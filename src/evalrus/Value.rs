use crate::evalrus::FatPtr::FatPtr;
use crate::evalrus::Ptrs::{FatPtr, ScopedPtr};
use crate::evalrus::TypeList::TypeList as T;


#[derive(Copy, Clone)]
pub enum Value<'guard> {
    ArrayU8(ScopedPtr<'guard, T::ArrayU8>),
    ArrayU16(ScopedPtr<'guard, T::ArrayU16>),
    ArrayU32(ScopedPtr<'guard, T::ArrayU32>),
    Dict(ScopedPtr<'guard, T::Dict>),
    Function(ScopedPtr<'guard, T::Function>),
    List(ScopedPtr<'guard, T::List>),
    Nil,
    Number(isize),
    NumberObject(ScopedPtr<'guard, T::NumberObject>),
    Pair(ScopedPtr<'guard, T::Pair>),
    Partial(ScopedPtr<'guard, T::Partial>),
    Symbol(ScopedPtr<'guard, T::Symbol>),
    Text(ScopedPtr<'guard, T::Text>),
    Upvalue(ScopedPtr<'guard, T::Upvalue>),
}

impl From<FatPtr> for Value<'guard> {
    fn from(ptr: FatPtr) -> Value<'guard> {


    }
}
