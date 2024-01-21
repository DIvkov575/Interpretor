use crate::evalrus::FatPtr;
use crate::internals::Alloc::RawPtr;


impl From<FatPtr> for TaggedPtr {
    fn from(ptr: FatPtr) -> TaggedPtr {
        match ptr {
            FatPtr::ArrayU8(raw) => TaggedPtr::object(raw),
            FatPtr::ArrayU16(raw) => TaggedPtr::object(raw),
            FatPtr::ArrayU32(raw) => TaggedPtr::object(raw),
            FatPtr::Dict(raw) => TaggedPtr::object(raw),
            FatPtr::Function(raw) => TaggedPtr::object(raw),
            FatPtr::List(raw) => TaggedPtr::object(raw),
            FatPtr::Nil => TaggedPtr::nil(),
            FatPtr::Number(value) => TaggedPtr::number(value),
            FatPtr::NumberObject(raw) => TaggedPtr::object(raw),
            FatPtr::Pair(raw) => TaggedPtr::pair(raw),
            FatPtr::Partial(raw) => TaggedPtr::object(raw),
            FatPtr::Text(raw) => TaggedPtr::object(raw),
            FatPtr::Symbol(raw) => TaggedPtr::symbol(raw),
            FatPtr::Upvalue(raw) => TaggedPtr::object(raw),
        }
    }
    fn into_fat_ptr(&self) -> crate::evalrus::FatPtr::FatPtr {
        unsafe {
            if self.tag == 0 {
                crate::evalrus::FatPtr::FatPtr::Nil
            } else {
                match get_tag(self.tag) {
                    TAG_NUMBER => crate::evalrus::FatPtr::FatPtr::Number(self.number >> 2),
                    TAG_SYMBOL => crate::evalrus::FatPtr::FatPtr::Symbol(RawPtr::untag(self.symbol)),
                    TAG_PAIR => crate::evalrus::FatPtr::FatPtr::Pair(RawPtr::untag(self.pair)),

                    TAG_OBJECT => {
                        let untyped_object_ptr = RawPtr::untag(self.object).as_untyped();
                        let header_ptr = HeapStorage::get_header(untyped_object_ptr);

                        header_ptr.as_ref().get_object_fatptr()
                    }

                    _ => panic!("Invalid TaggedPtr type tag!"),
                }
            }
        }
    }
}
