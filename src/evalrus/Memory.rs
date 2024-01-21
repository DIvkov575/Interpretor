use crate::evalrus::Heap::Heap;
use crate::evalrus::Mutator::Mutator;
use crate::evalrus::MutatorView;

pub struct Memory {
    heap: Heap,
}

impl Memory {
    pub fn mutate<M: Mutator>(&self, m: &M, input: M::Input) -> Result<M::Output, RuntimeError> {
        let mut guard = MutatorView::new(self);
        m.run(&mut guard, input)
    }

}
