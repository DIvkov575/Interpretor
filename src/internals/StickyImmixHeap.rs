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

        // TODO handle large objects
        if size_class == SizeClass::Large {
            // simply fail for objects larger than the block size
            return Err(AllocError::BadRequest);
        }

        let space = match blocks.head {
            // We already have a block to try to use...
            Some(ref mut head) => {
                // If this is a medium object that doesn't fit in the hole, use overflow
                if size_class == SizeClass::Medium && alloc_size > head.current_hole_size() {
                    return blocks.overflow_alloc(alloc_size);
                }

                // This is a small object that might fit in the current block...
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

            // We have no blocks to work with yet so make one
            None => {
                let mut head = BumpBlock::new()?;

                // earlier check for object size < block size should
                // mean we dont fail this expectation
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
        // calculate the total size of the object and it's header
        let header_size = size_of::<Self::Header>();
        let object_size = size_of::<T>();
        let total_size = header_size + object_size;

        // round the size to the next word boundary to keep objects aligned and get the size class
        // TODO BUG? should this be done separately for header and object?
        //  If the base allocation address is where the header gets placed, perhaps
        //  this breaks the double-word alignment object alignment desire?
        let alloc_size = alloc_size_of(total_size);
        let size_class = SizeClass::get_for_size(alloc_size)?;

        // attempt to allocate enough space for the header and the object
        let space = self.find_space(alloc_size, size_class)?;

        // instantiate an object header for type T, setting the mark bit to "allocated"
        let header = Self::Header::new::<T>(object_size as ArraySize, size_class, Mark::Allocated);

        // write the header into the front of the allocated space
        unsafe {
            write(space as *mut Self::Header, header);
        }

        // write the object into the allocated space after the header
        let object_space = unsafe { space.offset(header_size as isize) };
        unsafe {
            write(object_space as *mut T, object);
        }

        // return a pointer to the object in the allocated space
        Ok(RawPtr::new(object_space as *const T))
    }
    // ANCHOR_END: DefAlloc

    /// Allocate space for an array, creating an header for it, writing the header into the space
    /// and returning a pointer to the array space
    // ANCHOR: DefAllocArray
    fn alloc_array(&self, size_bytes: ArraySize) -> Result<RawPtr<u8>, AllocError> {
        // calculate the total size of the array and it's header
        let header_size = size_of::<Self::Header>();
        let total_size = header_size + size_bytes as usize;

        // round the size to the next word boundary to keep objects aligned and get the size class
        let alloc_size = alloc_size_of(total_size);
        let size_class = SizeClass::get_for_size(alloc_size)?;

        // attempt to allocate enough space for the header and the array
        let space = self.find_space(alloc_size, size_class)?;

        // instantiate an object header for an array, setting the mark bit to "allocated"
        let header = Self::Header::new_array(size_bytes, size_class, Mark::Allocated);

        // write the header into the front of the allocated space
        unsafe {
            write(space as *mut Self::Header, header);
        }

        // calculate where the array will begin after the header
        let array_space = unsafe { space.offset(header_size as isize) };

        // Initialize object_space to zero here.
        // If using the system allocator for any objects (SizeClass::Large, for example),
        // the memory may already be zeroed.
        let array = unsafe { from_raw_parts_mut(array_space as *mut u8, size_bytes as usize) };
        // The compiler should recognize this as optimizable
        for byte in array {
            *byte = 0;
        }

        // return a pointer to the array in the allocated space
        Ok(RawPtr::new(array_space as *const u8))
    }
    // ANCHOR_END: DefAllocArray

    /// Return the object header for a given object pointer
    // ANCHOR: DefGetHeader
    fn get_header(object: NonNull<()>) -> NonNull<Self::Header> {
        unsafe { NonNull::new_unchecked(object.cast::<Self::Header>().as_ptr().offset(-1)) }
    }
    // ANCHOR_END: DefGetHeader

    /// Return the object from it's header address
    // ANCHOR: DefGetObject
    fn get_object(header: NonNull<Self::Header>) -> NonNull<()> {
        unsafe { NonNull::new_unchecked(header.as_ptr().offset(1).cast::<()>()) }
    }
    // ANCHOR_END: DefGetObject
}


