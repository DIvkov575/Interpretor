use std::cell::Cell;
use std::ptr::read;
use crate::evalrus::MutatorView::MutatorView;
use crate::evalrus::Ptrs::ScopedPtr;
use crate::evalrus::Traits::MutatorScope;
use crate::evalrus::TypeList::TypeList;
use crate::frontend::RawArray::RawArray;
use crate::internals::Alloc::AllocObject;
use crate::internals::Errors::RuntimeError;

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


