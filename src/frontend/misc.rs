use std::hash::{BuildHasherDefault, Hash};
use crate::evalrus::Ptrs::TaggedScopedPtr;
use crate::evalrus::Traits::MutatorScope;
use crate::evalrus::Value::Value;
use crate::internals::Errors::RuntimeError;

