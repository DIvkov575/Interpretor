use crate::evalrus::Traits::MutatorScope;
use crate::evalrus::TypeList::TypeList;
use crate::internals::Alloc::AllocObject;

#[derive(Copy, Clone)]
pub struct Symbol {
    name_ptr: *const u8,
    name_len: usize,
}

impl Symbol {
    pub unsafe fn unguarded_as_str<'desired_lifetime>(&self) -> &'desired_lifetime str {
        let slice = std::slice::from_raw_parts(self.name_ptr, self.name_len);
        str::from_utf8(slice).unwrap()
    }

    pub fn as_str<'guard>(&self, _guard: &'guard dyn MutatorScope) -> &'guard str {
        unsafe { self.unguarded_as_str() }
    }
}

impl AllocObject<TypeList> for Symbol {
    const TYPE_ID: TypeList = TypeList::Symbol;
}
