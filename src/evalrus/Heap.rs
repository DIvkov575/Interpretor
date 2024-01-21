use crate::evalrus::Ptrs::{ScopedPtr, ScopedRef};
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
}


