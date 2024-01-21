use std::io::ErrorKind;
use std::mem::size_of;
use std::ptr::NonNull;
use crate::evalrus::MutatorView::MutatorView;
use crate::internals::Errors::RuntimeError;

pub struct RawArray<T: Sized> {
    /// Count of T-sized objects that can fit in the array
    capacity: ArraySize,
    ptr: Option<NonNull<T>>,
}

impl<T> RawArray<T> {
    pub fn with_capacity<'scope>(
        mem: &'scope MutatorView,
        capacity: u32,
    ) -> Result<RawArray<T>, RuntimeError> {
        // convert to bytes, checking for possible overflow of ArraySize limit
        let capacity_bytes = capacity
            .checked_mul(size_of::<T>() as ArraySize)
            .ok_or(RuntimeError::new(ErrorKind::BadAllocationRequest))?; //TODO!: runtimeerror::new()?

        Ok(RawArray {
            capacity,
            ptr: NonNull::new(mem.alloc_array(capacity_bytes)?.as_ptr() as *mut T),
        })
    }

}

impl<T: Sized> RawArray<T> {
    pub fn capacity(&self) -> ArraySize {
        self.capacity
    }

    pub fn as_ptr(&self) -> Option<*const T> {
        match self.ptr {
            Some(ptr) => Some(ptr.as_ptr()),
            None => None,
        }
    }
}
j