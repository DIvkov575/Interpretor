use std::cell::UnsafeCell;
use std::marker::PhantomData;
use std::mem::{replace, size_of};
use std::ptr::{NonNull, write};
use crate::frontend::Array::ArraySize;
use std::slice::from_raw_parts_mut;
use crate::internals::Alloc::{alloc_size_of, AllocError, AllocHeader, AllocObject, AllocRaw, Mark, RawPtr, SizeClass};
use crate::internals::BlockList::BlockList;
use crate::internals::BumpBlock::BumpBlock;

pub struct StickyImmixHeap<H> {
    blocks: UnsafeCell<BlockList>,

    _header_type: PhantomData<*const H>,
}

impl<H> StickyImmixHeap<H> {
    pub fn new() -> StickyImmixHeap<H> {
        StickyImmixHeap {
            blocks: UnsafeCell::new(BlockList::new()),
            _header_type: PhantomData,
        }
    }

    fn find_space(
        &self,
        alloc_size: usize,
        size_class: SizeClass,
    ) -> Result<*const u8, AllocError> {
        let blocks = unsafe { &mut *self.blocks.get() };


        if size_class == SizeClass::Large {
            return Err(AllocError::BadRequest);
        }

        let space = match blocks.head {
.
            Some(ref mut head) => {
                // If this is a medium object that doesn't fit in the hole, use overflow
                if size_class == SizeClass::Medium && alloc_size > head.current_hole_size() {
                    return blocks.overflow_alloc(alloc_size);
                }

                // small object that might fit 
                match head.inner_alloc(alloc_size) {
                    // the block has a suitable hole
                    Some(space) => space,

                    // the block does not have a suitable hole
                    None => {
                        let previous = replace(head, BumpBlock::new()?);

                        blocks.rest.push(previous);

                        head.inner_alloc(alloc_size).expect("Unexpected error!")
                    }
                }
            }


            None => {
                let mut head = BumpBlock::new()?;
                let space = head
                    .inner_alloc(alloc_size)
                    .expect("We expected this object to fit!");

                blocks.head = Some(head);

                space
            }
        } as *const u8;

        Ok(space)
    }
}

impl<H: AllocHeader> AllocRaw for StickyImmixHeap<H> {
    type Header = H;
    /// Allocate space for object `T`, creating an header for it and writing the object
    /// and the header into the space
    // ANCHOR: DefAlloc
    fn alloc<T>(&self, object: T) -> Result<RawPtr<T>, AllocError>
        where
            T: AllocObject<<Self::Header as AllocHeader>::TypeId>,
    {
        let header_size = size_of::<Self::Header>();
        let object_size = size_of::<T>();
        let total_size = header_size + object_size;

        // round the size to the next word boundary to keep objects aligned and get the size class
        // TODO BUG? should this be done separately for header and object?
        //  If the base allocation address is where the header gets placed, perhaps
        //  this breaks the double-word alignment object alignment desire?
        let alloc_size = alloc_size_of(total_size);
        let size_class = SizeClass::get_for_size(alloc_size)?;
        let space = self.find_space(alloc_size, size_class)?;
        let header = Self::Header::new::<T>(object_size as ArraySize, size_class, Mark::Allocated);
        unsafe { write(space as *mut Self::Header, header); }
        let object_space = unsafe { space.offset(header_size as isize) };
        unsafe { write(object_space as *mut T, object); }
        Ok(RawPtr::new(object_space as *const T))
    }
    /// Allocate space for an array, creating an header for it, writing the header into the space
    /// and returning a pointer to the array space
    // ANCHOR: DefAllocArray
    fn alloc_array(&self, size_bytes: ArraySize) -> Result<RawPtr<u8>, AllocError> {
        let header_size = size_of::<Self::Header>();
        let total_size = header_size + size_bytes as usize;

        // round the size to the next word boundary to keep objects aligned and get the size class
        let alloc_size = alloc_size_of(total_size);
        let size_class = SizeClass::get_for_size(alloc_size)?;
        let space = self.find_space(alloc_size, size_class)?;
        let header = Self::Header::new_array(size_bytes, size_class, Mark::Allocated);
        unsafe { write(space as *mut Self::Header, header); }
        let array_space = unsafe { space.offset(header_size as isize) };

        // Initialize object_space to zero here.
        // If using the system allocator for any objects (SizeClass::Large, for example),
        // the memory may already be zeroed.
        let array = unsafe { from_raw_parts_mut(array_space as *mut u8, size_bytes as usize) };
        // The compiler should recognize this as optimizable
        for byte in array {
            *byte = 0;
        }

        Ok(RawPtr::new(array_space as *const u8))
    }
    
    fn get_header(object: NonNull<()>) -> NonNull<Self::Header> {
        unsafe { NonNull::new_unchecked(object.cast::<Self::Header>().as_ptr().offset(-1)) }
    }
    
    fn get_object(header: NonNull<Self::Header>) -> NonNull<()> {
        unsafe { NonNull::new_unchecked(header.as_ptr().offset(1).cast::<()>()) }
    }
}


