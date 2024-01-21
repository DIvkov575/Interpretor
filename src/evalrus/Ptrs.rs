use std::cell::Cell;
use std::ptr::NonNull;
use crate::internals::Alloc::RawPtr;
use crate::evalrus::TypeList::TypeList::*;
use crate::evalrus::Value::Value;


const TAG_MASK: usize = 0x3;
pub const TAG_SYMBOL: usize = 0x0;
pub const TAG_PAIR: usize = 0x1;
pub const TAG_OBJECT: usize = 0x2;
pub const TAG_NUMBER: usize = 0x3;
const PTR_MASK: usize = !0x3;


#[derive(Clone)]
pub struct CellPtr<T: Sized> {
    inner: Cell<RawPtr<T>>,
}

pub struct ScopedPtr<'guard, T: Sized> {
    value: &'guard T,
}

pub trait ScopedRef<T> {
    fn scoped_ref<'scope>(&self, guard: &'scope dyn MutatorScope) -> &'scope T;
}

impl<T> ScopedRef<T> for RawPtr<T> {
    fn scoped_ref<'scope>(&self, _guard: &'scope dyn MutatorScope) -> &'scope T {
        unsafe { &*self.as_ptr() }
    }
}

impl<T: Sized> CellPtr<T> {
    pub fn get<'guard>(&self, guard: &'guard dyn MutatorScope) -> ScopedPtr<'guard, T> {
        ScopedPtr::new(guard, self.inner.get().scoped_ref(guard))
    }
}



