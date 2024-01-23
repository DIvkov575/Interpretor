use std::cell::Cell;
use std::ptr::read;
use crate::evalrus::MutatorView::MutatorView;
use crate::evalrus::Ptrs::{ScopedPtr, TaggedCellPtr, TaggedScopedPtr};
use crate::evalrus::Traits::MutatorScope;
use crate::evalrus::TypeList::TypeList;
use crate::frontend::RawArray::RawArray;
use crate::frontend::Traits::{Container, FillContainer, StackContainer};
use crate::internals::Alloc::AllocObject;
use crate::internals::Errors::{ErrorKind, RuntimeError};


pub type List = Array<TaggedCellPtr>;

pub type ArraySize = u32;
pub type BorrowFlag = isize;
const INTERIOR_ONLY: isize = 0;
const EXPOSED_MUTABLY: isize = 1;



pub type ArrayU8 = Array<u8>;
pub type ArrayU16 = Array<u16>;
pub type ArrayU32 = Array<u32>;


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

impl<T: Sized + Clone> Container<T> for Array<T> {
    fn new() -> Array<T> {
        Array {
            length: Cell::new(0),
            data: Cell::new(RawArray::new()),
            borrow: Cell::new(INTERIOR_ONLY),
        }
    }

    fn with_capacity<'guard>(
        mem: &'guard MutatorView,
        capacity: ArraySize,
    ) -> Result<Array<T>, RuntimeError> {
        Ok(Array {
            length: Cell::new(0),
            data: Cell::new(RawArray::with_capacity(mem, capacity)?),
            borrow: Cell::new(INTERIOR_ONLY),
        })
    }

    fn clear<'guard>(&self, _guard: &'guard MutatorView) -> Result<(), RuntimeError> {
        if self.borrow.get() != INTERIOR_ONLY {
            Err(RuntimeError::new(ErrorKind::MutableBorrowError))
        } else {
            self.length.set(0);
            Ok(())
        }
    }

    fn length(&self) -> ArraySize {
        self.length.get()
    }
}


impl<T: Sized + Clone> FillContainer<T> for Array<T> {
    fn fill<'guard>(
        &self,
        mem: &'guard MutatorView,
        size: ArraySize,
        item: T,
    ) -> Result<(), RuntimeError> {
        let length = self.length();

        if length > size {
            Ok(())
        } else {
            let mut array = self.data.get(); // Takes a copy

            let capacity = array.capacity();

            if size > capacity {
                if capacity == 0 {
                    array.resize(mem, DEFAULT_ARRAY_SIZE)?;
                } else {
                    array.resize(mem, default_array_growth(capacity)?)?;
                }
                // Replace the struct's copy with the resized RawArray object
                self.data.set(array);
            }

            self.length.set(size);

            for index in length..size {
                self.write(mem, index, item.clone())?;
            }

            Ok(())
        }
    }
}


impl<T: Sized + Clone> StackContainer<T> for Array<T> {
    /// Push can trigger an underlying array resize, hence it requires the ability to allocate
    // ANCHOR: DefStackContainerArrayPush
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
    fn pop<'guard>(&self, guard: &'guard dyn MutatorScope) -> Result<T, RuntimeError> {
        if self.borrow.get() != INTERIOR_ONLY {
            return Err(RuntimeError::new(ErrorKind::MutableBorrowError));
        }

        let length = self.length.get();

        if length == 0 {
            Err(RuntimeError::new(ErrorKind::BoundsError))
        } else {
            let last = length - 1;
            let item = self.read(guard, last)?;
            self.length.set(last);
            Ok(item)
        }
    }

    /// Return the value at the top of the stack without removing it
    fn top<'guard>(&self, guard: &'guard dyn MutatorScope) -> Result<T, RuntimeError> {
        let length = self.length.get();

        if length == 0 {
            Err(RuntimeError::new(ErrorKind::BoundsError))
        } else {
            let last = length - 1;
            let item = self.read(guard, last)?;
            Ok(item)
        }
    }
}



pub trait StackAnyContainer: StackContainer<TaggedCellPtr> {
    /// Push can trigger an underlying array resize, hence it requires the ability to allocate
    fn push<'guard>(
        &self,
        mem: &'guard MutatorView,
        item: TaggedScopedPtr<'guard>,
    ) -> Result<(), RuntimeError>;

    /// Pop returns a bounds error if the container is empty, otherwise moves the last item of the
    /// array out to the caller.
    fn pop<'guard>(
        &self,
        _guard: &'guard dyn MutatorScope,
    ) -> Result<TaggedScopedPtr<'guard>, RuntimeError>;

    /// Return the value at the top of the stack without removing it
    fn top<'guard>(
        &self,
        _guard: &'guard dyn MutatorScope,
    ) -> Result<TaggedScopedPtr<'guard>, RuntimeError>;
}
