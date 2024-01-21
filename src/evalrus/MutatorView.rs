use crate::evalrus::FatPtr::FatPtr;
use crate::evalrus::Ptrs::{ScopedPtr, ScopedRef, TaggedScopedPtr};
use crate::evalrus::Heap::Heap;
use crate::evalrus::TypeList::TypeList;
use crate::internals::Alloc::{AllocObject, RawPtr};

pub struct MutatorView<'memory> {
    heap: &'memory Heap,
}

impl<'memory> MutatorView<'memory> {
    pub fn alloc<T>(&self, object: T) -> Result<ScopedPtr<'_, T>, RuntimeError>
        where
            T: AllocObject<TypeList>,
    {
        Ok(ScopedPtr::new(
            self,
            self.heap.alloc(object)?.scoped_ref(self),
        ))
    }
}
impl MutatorView {
    pub fn alloc_tagged<T>(&self, object: T) -> Result<TaggedScopedPtr<'_>, RuntimeError>
        where
            FatPtr: From<RawPtr<T>>,
            T: AllocObject<TypeList>,
    {
        Ok(TaggedScopedPtr::new(self, self.heap.alloc_tagged(object)?))
    }
}

