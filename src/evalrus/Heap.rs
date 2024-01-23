use crate::evalrus::ObjectHeader::ObjectHeader;
use crate::evalrus::Ptrs::{FatPtr, ScopedPtr, ScopedRef, TaggedPtr};
use crate::evalrus::SymbolMap::SymbolMap;
use crate::evalrus::TypeList::TypeList;
use crate::internals::Alloc::{AllocObject, AllocRaw, RawPtr};
use crate::internals::Errors::RuntimeError;
use crate::internals::StickyImmixHeap::StickyImmixHeap;

pub type HeapStorage = StickyImmixHeap<ObjectHeader>;
pub struct Heap {
    heap: HeapStorage,
    syms: SymbolMap,
}

impl Heap {
    pub fn alloc<T>(&self, object: T) -> Result<RawPtr<T>, RuntimeError>
        where
            T: AllocObject<TypeList>,
    {
        Ok(self.heap.alloc(object)?)
    }
    pub fn alloc_tagged<T>(&self, object: T) -> Result<TaggedPtr, RuntimeError>
        where
            FatPtr: From<RawPtr<T>>,
            T: AllocObject<TypeList>,
    {
        Ok(TaggedPtr::from(FatPtr::from(self.heap.alloc(object)?)))
    }

    pub fn lookup_sym(&self, name: &str) -> TaggedPtr {
        TaggedPtr::symbol(self.syms.lookup(name))
    }

}

