use std::hash::Hasher;
use crate::evalrus::MutatorView::MutatorView;
use crate::evalrus::Ptrs::{ScopedPtr, TaggedCellPtr, TaggedScopedPtr};
use crate::evalrus::Traits::MutatorScope;
use crate::frontend::Array::ArraySize;
use crate::internals::Errors::RuntimeError;

pub trait FillContainer<T: Sized + Clone>: Container<T> {
    /// The `item` is an object to copy into each container memory slot.
    fn fill<'guard>(
        &self,
        mem: &'guard MutatorView,
        size: ArraySize,
        item: T,
    ) -> Result<(), RuntimeError>;
}


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

pub trait IndexedContainer<T: Sized + Clone>: Container<T> {
    /// Return a copy of the object at the given index. Bounds-checked.
    fn get<'guard>(
        &self,
        _guard: &'guard dyn MutatorScope,
        index: ArraySize,
    ) -> Result<T, RuntimeError>;

    /// Move an object into the array at the given index. Bounds-checked.
    fn set<'guard>(
        &self,
        _guard: &'guard dyn MutatorScope,
        index: ArraySize,
        item: T,
    ) -> Result<(), RuntimeError>;
}


pub trait Container<T: Sized + Clone>: Sized {
    /// Create a new, empty container instance.
    fn new() -> Self;
    fn with_capacity<'guard>(
        mem: &'guard MutatorView,
        capacity: ArraySize,
    ) -> Result<Self, RuntimeError>;

    /// Reset the size of the container to zero - empty
    fn clear<'guard>(&self, mem: &'guard MutatorView) -> Result<(), RuntimeError>;

    /// Count of items in the container
    fn length(&self) -> ArraySize;
}

/// Similar to Hash but for use in a mutator lifetime-limited scope

pub trait Hashable {
    fn hash<'guard, H: Hasher>(&self, _guard: &'guard dyn MutatorScope, hasher: &mut H);
}

pub trait HashIndexedAnyContainer {
    /// Return a pointer to to the object associated with the given key.
    /// Absence of an association should return an error.
    fn lookup<'guard>(
        &self,
        guard: &'guard dyn MutatorScope,
        key: TaggedScopedPtr,
    ) -> Result<TaggedScopedPtr<'guard>, RuntimeError>;

    /// Associate a key with a value.
    fn assoc<'guard>(
        &self,
        mem: &'guard MutatorView,
        key: TaggedScopedPtr<'guard>,
        value: TaggedScopedPtr<'guard>,
    ) -> Result<(), RuntimeError>;

    /// Remove an association by its key.
    fn dissoc<'guard>(
        &self,
        guard: &'guard dyn MutatorScope,
        key: TaggedScopedPtr,
    ) -> Result<TaggedScopedPtr<'guard>, RuntimeError>;

    /// Returns true if the key exists in the container.
    fn exists<'guard>(
        &self,
        guard: &'guard dyn MutatorScope,
        key: TaggedScopedPtr,
    ) -> Result<bool, RuntimeError>;
}
// ANCHOR_END: DefHashIndexedAnyContainer

/// Convert a Pair list to a different container
pub trait AnyContainerFromPairList: Container<TaggedCellPtr> {
    fn from_pair_list<'guard>(
        &self,
        mem: &'guard MutatorView,
        pair_list: TaggedScopedPtr<'guard>,
    ) -> Result<(), RuntimeError>;
}

/// Replace the contents of a container with the values in the slice
pub trait ContainerFromSlice<T: Sized + Clone>: Container<T> {
    fn from_slice<'guard>(
        mem: &'guard MutatorView,
        data: &[T],
    ) -> Result<ScopedPtr<'guard, Self>, RuntimeError>;
}
