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




impl From<FatPtr> for TaggedPtr {
    fn from(ptr: FatPtr) -> TaggedPtr {
        match ptr {
            FatPtr::ArrayU8(raw) => TaggedPtr::object(raw),
            FatPtr::ArrayU16(raw) => TaggedPtr::object(raw),
            FatPtr::ArrayU32(raw) => TaggedPtr::object(raw),
            FatPtr::Dict(raw) => TaggedPtr::object(raw),
            FatPtr::Function(raw) => TaggedPtr::object(raw),
            FatPtr::List(raw) => TaggedPtr::object(raw),
            FatPtr::Nil => TaggedPtr::nil(),
            FatPtr::Number(value) => TaggedPtr::number(value),
            FatPtr::NumberObject(raw) => TaggedPtr::object(raw),
            FatPtr::Pair(raw) => TaggedPtr::pair(raw),
            FatPtr::Partial(raw) => TaggedPtr::object(raw),
            FatPtr::Text(raw) => TaggedPtr::object(raw),
            FatPtr::Symbol(raw) => TaggedPtr::symbol(raw),
            FatPtr::Upvalue(raw) => TaggedPtr::object(raw),
        }
    }
    fn into_fat_ptr(&self) -> crate::evalrus::FatPtr::FatPtr {
        unsafe {
            if self.tag == 0 {
                crate::evalrus::FatPtr::FatPtr::Nil
            } else {
                match get_tag(self.tag) {
                    TAG_NUMBER => crate::evalrus::FatPtr::FatPtr::Number(self.number >> 2),
                    TAG_SYMBOL => crate::evalrus::FatPtr::FatPtr::Symbol(RawPtr::untag(self.symbol)),
                    TAG_PAIR => crate::evalrus::FatPtr::FatPtr::Pair(RawPtr::untag(self.pair)),

                    TAG_OBJECT => {
                        let untyped_object_ptr = RawPtr::untag(self.object).as_untyped();
                        let header_ptr = HeapStorage::get_header(untyped_object_ptr);

                        header_ptr.as_ref().get_object_fatptr()
                    }

                    _ => panic!("Invalid TaggedPtr type tag!"),
                }
            }
        }
    }
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

impl From<TaggedPtr> for FatPtr {
    fn from(ptr: TaggedPtr) -> FatPtr {
        ptr.into_fat_ptr()
    }
}

impl TaggedPtr {

}
