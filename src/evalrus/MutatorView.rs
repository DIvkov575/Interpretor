use crate::evalrus::Ptrs::{ScopedPtr, ScopedRef};
use crate::evalrus::Heap::Heap;
use crate::internals::Alloc::AllocObject;

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
