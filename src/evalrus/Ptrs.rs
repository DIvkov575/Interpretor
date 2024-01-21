use std::cell::Cell;
use std::ptr::NonNull;
use crate::internals::Alloc::RawPtr;
use crate::evalrus::TypeList::TypeList::*;
use crate::evalrus::Value::Value;


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

#[derive(Copy, Clone)]
pub union TaggedPtr {
    tag: usize,
    number: isize,
    symbol: NonNull<Symbol>,
    pair: NonNull<Pair>,
    object: NonNull<()>,
}


#[derive(Copy, Clone)]
pub enum FatPtr {
    ArrayU8(RawPtr<ArrayU8>),
    ArrayU16(RawPtr<ArrayU16>),
    ArrayU32(RawPtr<ArrayU32>),
    Dict(RawPtr<Dict>),
    Function(RawPtr<Function>),
    List(RawPtr<List>),
    Nil,
    Number(isize),
    NumberObject(RawPtr<NumberObject>),
    Pair(RawPtr<Pair>),
    Partial(RawPtr<Partial>),
    Symbol(RawPtr<Symbol>),
    Text(RawPtr<Text>),
    Upvalue(RawPtr<Upvalue>),
}
impl FatPtr {
    pub fn as_value<'guard>(&self, guard: &'guard dyn MutatorScope) -> Value<'guard> {
        match self {
            FatPtr::ArrayU8(raw_ptr) => {
                Value::ArrayU8(ScopedPtr::new(guard, raw_ptr.scoped_ref(guard)))
            }
            FatPtr::ArrayU16(raw_ptr) => {
                Value::ArrayU16(ScopedPtr::new(guard, raw_ptr.scoped_ref(guard)))
            }
            FatPtr::ArrayU32(raw_ptr) => {
                Value::ArrayU32(ScopedPtr::new(guard, raw_ptr.scoped_ref(guard)))
            }
            FatPtr::Dict(raw_ptr) => Value::Dict(ScopedPtr::new(guard, raw_ptr.scoped_ref(guard))),
            FatPtr::Function(raw_ptr) => {
                Value::Function(ScopedPtr::new(guard, raw_ptr.scoped_ref(guard)))
            }
            FatPtr::List(raw_ptr) => Value::List(ScopedPtr::new(guard, raw_ptr.scoped_ref(guard))),
            FatPtr::Nil => Value::Nil,
            FatPtr::Number(num) => Value::Number(*num),
            FatPtr::NumberObject(raw_ptr) => {
                Value::NumberObject(ScopedPtr::new(guard, raw_ptr.scoped_ref(guard)))
            }
            FatPtr::Pair(raw_ptr) => Value::Pair(ScopedPtr::new(guard, raw_ptr.scoped_ref(guard))),
            FatPtr::Partial(raw_ptr) => {
                Value::Partial(ScopedPtr::new(guard, raw_ptr.scoped_ref(guard)))
            }
            FatPtr::Symbol(raw_ptr) => {
                Value::Symbol(ScopedPtr::new(guard, raw_ptr.scoped_ref(guard)))
            }
            FatPtr::Text(raw_ptr) => Value::Text(ScopedPtr::new(guard, raw_ptr.scoped_ref(guard))),
            FatPtr::Upvalue(raw_ptr) => {
                Value::Upvalue(ScopedPtr::new(guard, raw_ptr.scoped_ref(guard)))
            }
        }
    }
}
