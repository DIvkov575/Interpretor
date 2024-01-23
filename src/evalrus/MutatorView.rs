use crate::evalrus::FatPtr::FatPtr;
use crate::evalrus::Ptrs::{ScopedPtr, ScopedRef, TaggedScopedPtr};
use crate::evalrus::Heap::Heap;
use crate::evalrus::TypeList::TypeList;
use crate::internals::Alloc::{AllocObject, RawPtr};
use crate::internals::Errors::RuntimeError;

pub struct MutatorView<'memory> {
    pub heap: &'memory Heap,
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


impl<'memory> MutatorView<'memory> {
    pub fn lookup_sym(&self, name: &str) -> TaggedScopedPtr<'_> {
        TaggedScopedPtr::new(self, self.heap.lookup_sym(name))
    }
}