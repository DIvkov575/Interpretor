use crate::evalrus::Ptrs::{ScopedPtr, ScopedRef, TaggedPtr};
use crate::evalrus::Traits::MutatorScope;
use crate::internals::Alloc::RawPtr;
use crate::evalrus::TypeList::TypeList::*;
use crate::evalrus::Value::Value;

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

impl From<TaggedPtr> for FatPtr {
    fn from(ptr: TaggedPtr) -> FatPtr {
        ptr.into_fat_ptr()
    }
}
