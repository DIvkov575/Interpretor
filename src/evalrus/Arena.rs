use crate::internals::StickyImmixHeap::StickyImmixHeap;

pub struct Arena {
    heap: StickyImmixHeap<ArenaHeader>,
}
