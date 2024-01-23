use crate::evalrus::TypeList::TypeList;
use crate::frontend::Array::ArraySize;
use crate::internals::Alloc::{AllocHeader, AllocObject, Mark, SizeClass};
use crate::internals::StickyImmixHeap::StickyImmixHeap;

pub struct Arena {
    heap: StickyImmixHeap<ArenaHeader>,
}

pub struct ArenaHeader {}

/// Since we're not using this functionality in an Arena, the impl is just
/// a set of no-ops.
impl AllocHeader for ArenaHeader {
    type TypeId = TypeList;

    fn new<O: AllocObject<Self::TypeId>>(
        _size: u32,
        _size_class: SizeClass,
        _mark: Mark,
    ) -> ArenaHeader {
        ArenaHeader {}
    }

    fn new_array(_size: ArraySize, _size_class: SizeClass, _mark: Mark) -> ArenaHeader {
        ArenaHeader {}
    }

    fn mark(&mut self) {}

    fn is_marked(&self) -> bool {
        true
    }

    fn size_class(&self) -> SizeClass {
        SizeClass::Small
    }

    fn size(&self) -> u32 {
        1
    }

    fn type_id(&self) -> TypeList {
        TypeList::Symbol
    }
}
