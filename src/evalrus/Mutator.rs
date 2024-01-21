use crate::evalrus::MutatorView::MutatorView;

pub trait Mutator: Sized {
    type Input;
    type Output;

    fn run(&self, mem: &MutatorView, input: Self::Input) -> Result<Self::Output, RuntimeError>;

    // TODO
    // function to return iterator that iterates over roots
}
