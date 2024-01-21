use std::cell::UnsafeCell;
use std::marker::PhantomData;
use std::mem::{replace, size_of};
use std::ptr::{from_raw_parts_mut, NonNull, write};
use crate::Alloc::{AllocError, AllocHeader, AllocObject, AllocRaw, RawPtr};
use crate::AllocError;
use crate::BlockList::BlockList;
use crate::BumpBlock::BumpBlock;

pub struct StickyImmixHeap<H> {
    blocks: UnsafeCell<BlockList>,

    _header_type: PhantomData<*const H>,
}

impl StickyImmixHeap<H> {
    fn find_space(
        &self,
        alloc_size: usize,
        size_class: SizeClass,
    ) -> Result<*const u8, AllocError> {

        let blocks = unsafe { &mut *self.blocks.get() };
        if size_class == SizeClass::Medium && alloc_size > head.current_hole_size() {
            return blocks.overflow_alloc(alloc_size);
        }    }
}

impl<H: AllocHeader> AllocRaw for StickyImmixHeap<H> {
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
    fn get_header(object: NonNull<()>) -> NonNull<Self::Header> {
        unsafe { NonNull::new_unchecked(object.cast::<Self::Header>().as_ptr().offset(-1)) }
    }
    fn get_object(header: NonNull<Self::Header>) -> NonNull<()> {
        unsafe { NonNull::new_unchecked(header.as_ptr().offset(1).cast::<()>()) }
    }
}


