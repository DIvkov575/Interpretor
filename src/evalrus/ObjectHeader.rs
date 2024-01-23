use crate::evalrus::Heap::HeapStorage;
use crate::evalrus::Ptrs::FatPtr;
use crate::evalrus::TypeList::TypeList::{self, *};
use crate::internals::Alloc::{AllocRaw, Mark, RawPtr, SizeClass, Tagged};

pub struct ObjectHeader {
    mark: Mark,
    size_class: SizeClass,
    type_id: TypeList,
    size_bytes: u32,
}



impl ObjectHeader {
    pub unsafe fn get_object_fatptr(&self) -> FatPtr {
        let ptr_to_self = self.non_null_ptr();
        let object_addr = HeapStorage::get_object(ptr_to_self);

        match self.type_id {
            TypeList::ArrayU8 => FatPtr::ArrayU8(RawPtr::untag(object_addr.cast::<ArrayU8>())),
            TypeList::ArrayU16 => FatPtr::ArrayU16(RawPtr::untag(object_addr.cast::<ArrayU16>())),
            TypeList::ArrayU32 => FatPtr::ArrayU32(RawPtr::untag(object_addr.cast::<ArrayU32>())),
            TypeList::Dict => FatPtr::Dict(RawPtr::untag(object_addr.cast::<Dict>())),
            TypeList::Function => FatPtr::Function(RawPtr::untag(object_addr.cast::<Function>())),
            TypeList::List => FatPtr::List(RawPtr::untag(object_addr.cast::<List>())),
            TypeList::NumberObject => {
                FatPtr::NumberObject(RawPtr::untag(object_addr.cast::<NumberObject>()))
            }
            TypeList::Pair => FatPtr::Pair(RawPtr::untag(object_addr.cast::<Pair>())),
            TypeList::Partial => FatPtr::Partial(RawPtr::untag(object_addr.cast::<Partial>())),
            TypeList::Symbol => FatPtr::Symbol(RawPtr::untag(object_addr.cast::<Symbol>())),
            TypeList::Text => FatPtr::Text(RawPtr::untag(object_addr.cast::<Text>())),
            TypeList::Upvalue => FatPtr::Upvalue(RawPtr::untag(object_addr.cast::<Upvalue>())),

            // Other types not represented by FatPtr are an error to id here
            _ => panic!("Invalid ObjectHeader type tag {:?}!", self.type_id),
        }
    }
}
