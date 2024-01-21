use crate::evalrus::MutatorView::MutatorView;
use crate::evalrus::Ptrs::{TaggedCellPtr, TaggedScopedPtr};
use crate::evalrus::Traits::MutatorScope;
use crate::internals::Errors::RuntimeError;

pub trait StackContainer<T: Sized + Clone>: Container<T> {
    /// Push can trigger an underlying array resize, hence it requires the ability to allocate
    fn push<'guard>(&self, mem: &'guard MutatorView, item: T) -> Result<(), RuntimeError>;

    /// Pop returns a bounds error if the container is empty, otherwise moves the last item of the
    /// array out to the caller.
    fn pop<'guard>(&self, _guard: &'guard dyn MutatorScope) -> Result<T, RuntimeError>;

    /// Return the value at the top of the stack without removing it
    fn top<'guard>(&self, _guard: &'guard dyn MutatorScope) -> Result<T, RuntimeError>;
}


pub trait StackAnyContainer: StackContainer<TaggedCellPtr> {
    /// Push can trigger an underlying array resize, hence it requires the ability to allocate
    fn push<'guard>(
        &self,
        mem: &'guard MutatorView,
        item: TaggedScopedPtr<'guard>,
    ) -> Result<(), RuntimeError>;

    /// Pop returns a bounds error if the container is empty, otherwise moves the last item of the
    /// array out to the caller.
    fn pop<'guard>(
        &self,
        _guard: &'guard dyn MutatorScope,
    ) -> Result<TaggedScopedPtr<'guard>, RuntimeError>;

    /// Return the value at the top of the stack without removing it
    fn top<'guard>(
        &self,
        _guard: &'guard dyn MutatorScope,
    ) -> Result<TaggedScopedPtr<'guard>, RuntimeError>;
}
