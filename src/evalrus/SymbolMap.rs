use std::cell::RefCell;
use std::collections::HashMap;
use crate::evalrus::Arena::Arena;
use crate::evalrus::Symbol::Symbol;
use crate::internals::Alloc::RawPtr;

pub struct SymbolMap {
    map: RefCell<HashMap<String, RawPtr<Symbol>>>,
    arena: Arena,
}

impl SymbolMap {
    pub fn lookup(&self, name: &str) -> RawPtr<Symbol> {
        {
            if let Some(ptr) = self.map.borrow().get(name) {
                return *ptr;
            }
        }

        let name = String::from(name);
        let ptr = self.arena.alloc(Symbol::new(&name)).unwrap();
        self.map.borrow_mut().insert(name, ptr);
        ptr
    }
}

