use std::cell::Cell;
use std::ptr::NonNull;
use crate::evalrus::FatPtr::FatPtr;
use crate::evalrus::Heap::HeapStorage;
use crate::evalrus::Traits::MutatorScope;
use crate::internals::Alloc::{AllocRaw, RawPtr, Tagged};
use crate::evalrus::TypeList::TypeList::*;
use crate::evalrus::Value::Value;


const TAG_MASK: usize = 0x3;
pub const TAG_SYMBOL: usize = 0x0;
pub const TAG_PAIR: usize = 0x1;
pub const TAG_OBJECT: usize = 0x2;
pub const TAG_NUMBER: usize = 0x3;
const PTR_MASK: usize = !0x3;


#[derive(Clone, Debug)]
pub struct CellPtr<T: Sized> {
    inner: Cell<RawPtr<T>>,
}

#[derive(Debug, Clone, Copy)]
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


#[derive(Clone)]
pub struct TaggedCellPtr {
    inner: Cell<TaggedPtr>,
}

impl TaggedCellPtr {
    pub fn get<'guard>(&self, guard: &'guard dyn MutatorScope) -> TaggedScopedPtr<'guard> {
        TaggedScopedPtr::new(guard, self.inner.get())
    }
}


#[derive(Copy, Clone)]
pub struct TaggedScopedPtr<'guard> {
    ptr: TaggedPtr,
    value: Value<'guard>,
}


#[derive(Copy, Clone, Debug)]
pub union TaggedPtr {
    tag: usize,
    number: isize,
    symbol: NonNull<Symbol>,
    pair: NonNull<Pair>,
    object: NonNull<()>,
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
}



impl TaggedPtr {
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

pub fn get_tag(tagged_word: usize) -> usize {
    tagged_word & TAG_MASK
}
