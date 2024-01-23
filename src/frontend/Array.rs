use std::cell::Cell;
use std::ptr::read;
use crate::evalrus::MutatorView::MutatorView;
use crate::evalrus::Ptrs::{ScopedPtr, TaggedCellPtr, TaggedScopedPtr};
use crate::evalrus::Traits::MutatorScope;
use crate::evalrus::TypeList::TypeList;
use crate::frontend::RawArray::RawArray;
use crate::frontend::Traits::{StackAnyContainer, StackContainer};
use crate::internals::Alloc::AllocObject;
use crate::internals::Errors::RuntimeError;


pub type List = Array<TaggedCellPtr>;

pub type ArraySize = u32;
pub type BorrowFlag = isize;

pub const DEFAULT_ARRAY_SIZE: ArraySize = 8;
pub fn default_array_growth(capacity: ArraySize) -> Result<ArraySize, RuntimeError> {
    if capacity == 0 {
        Ok(DEFAULT_ARRAY_SIZE)
    } else {
        capacity
            .checked_add(capacity / 2)
            .ok_or(RuntimeError::new(ErrorKind::BadAllocationRequest))
    }
}

#[derive(Clone)]
pub struct Array<T: Sized + Clone> {
    length: Cell<ArraySize>,
    data: Cell<RawArray<T>>,
    borrow: Cell<BorrowFlag>,
}

impl<T: Sized + Clone> Array<T> {

    pub fn alloc<'guard>(
        mem: &'guard MutatorView,
    ) -> Result<ScopedPtr<'guard, Array<T>>, RuntimeError>
        where
            Array<T>: AllocObject<TypeList>,
    {
        mem.alloc(Array::new())
    }

    fn get_offset(&self, index: ArraySize) -> Result<*mut T, RuntimeError> {
        if index >= self.length.get() {
            Err(RuntimeError::new(ErrorKind::BoundsError))
        } else {
            let ptr = self
                .data
                .get()
                .as_ptr()
                .ok_or_else(|| RuntimeError::new(ErrorKind::BoundsError))?;

            let dest_ptr = unsafe { ptr.offset(index as isize) as *mut T };

            Ok(dest_ptr)
        }
    }
    fn read<'guard>(
        &self,
        _guard: &'guard dyn MutatorScope,
        index: ArraySize,
    ) -> Result<T, RuntimeError> {
        unsafe {
            let dest = self.get_offset(index)?;
            Ok(read(dest))
        }
    }
    pub fn read_ref<'guard>(
            &self,
            _guard: &'guard dyn MutatorScope,
            index: ArraySize,
        ) -> Result<&T, RuntimeError> {
            unsafe {
                let dest = self.get_offset(index)?;
                Ok(&*dest as &T)
            }
        }
    }

}


impl<T: Sized + Clone> StackContainer<T> for Array<T> {
    fn push<'guard>(&self, mem: &'guard MutatorView, item: T) -> Result<(), RuntimeError> {
        if self.borrow.get() != INTERIOR_ONLY {
            return Err(RuntimeError::new(ErrorKind::MutableBorrowError));
        }

        let length = self.length.get();
        let mut array = self.data.get(); // Takes a copy

        let capacity = array.capacity();

        if length == capacity {
            if capacity == 0 {
                array.resize(mem, DEFAULT_ARRAY_SIZE)?;
            } else {
                array.resize(mem, default_array_growth(capacity)?)?;
            }
            // Replace the struct's copy with the resized RawArray object
            self.data.set(array);
        }

        self.length.set(length + 1);
        self.write(mem, length, item)?;
        Ok(())
    }
}


impl StackAnyContainer for Array<TaggedCellPtr> {
    fn push<'guard>(
        &self,
        mem: &'guard MutatorView,
        item: TaggedScopedPtr<'guard>,
    ) -> Result<(), RuntimeError> {
        StackContainer::<TaggedCellPtr>::push(self, mem, TaggedCellPtr::new_with(item))
    }
}
