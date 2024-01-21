use crate::evalrus::Ptrs::{FatPtr, ScopedPtr, ScopedRef, TaggedPtr};
use crate::evalrus::TypeList::TypeList;
use crate::internals::Alloc::{AllocObject, AllocRaw, RawPtr};
use crate::internals::StickyImmixHeap::StickyImmixHeap;

pub type HeapStorage = StickyImmixHeap<ObjectHeader>;
pub struct Heap {
    heap: HeapStorage,
    syms: SymbolMap,
}

impl Heap {
    pub(crate) fn alloc<T>(&self, object: T) -> Result<RawPtr<T>, RuntimeError>
        where
            T: AllocObject<TypeList>,
    {
        Ok(self.heap.alloc(object)?)
    }
    fn alloc_tagged<T>(&self, object: T) -> Result<TaggedPtr, RuntimeError>
        where
            FatPtr: From<RawPtr<T>>,
            T: AllocObject<TypeList>,
    {
        Ok(TaggedPtr::from(FatPtr::from(self.heap.alloc(object)?)))
    }


}


