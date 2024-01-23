use crate::evalrus::Heap::Heap;
use crate::evalrus::MutatorView;
use crate::evalrus::Traits::Mutator;
use crate::internals::Errors::RuntimeError;

pub struct Memory {
    heap: Heap,
}

impl Memory {
    pub fn mutate<M: Mutator>(&self, m: &M, input: M::Input) -> Result<M::Output, RuntimeError> {
        let mut guard = MutatorView::MutatorView::new(self);
        m.run(&mut guard, input)
    }

}
