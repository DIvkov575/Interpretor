use crate::evalrus::MutatorView::MutatorView;
use crate::evalrus::Ptrs::TaggedScopedPtr;
use crate::frontend::Token::Token;
use crate::internals::Errors::RuntimeError;

pub fn tokenize(input: &str) -> Result<Vec<Token>, RuntimeError>;

fn parse_tokens<'guard>(
    mem: &'guard MutatorView,
    tokens: Vec<Token>,
) -> Result<TaggedScopedPtr<'guard>, RuntimeError>;

pub fn parse<'guard>(
    mem: &'guard MutatorView,
    input: &str,
) -> Result<TaggedScopedPtr<'guard>, RuntimeError> {
    parse_tokens(mem, tokenize(input)?)
}


