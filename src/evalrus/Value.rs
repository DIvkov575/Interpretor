#[derive(Copy, Clone)]
pub enum Value<'guard> {
    ArrayU8(ScopedPtr<'guard, ArrayU8>),
    ArrayU16(ScopedPtr<'guard, ArrayU16>),
    ArrayU32(ScopedPtr<'guard, ArrayU32>),
    Dict(ScopedPtr<'guard, Dict>),
    Function(ScopedPtr<'guard, Function>),
    List(ScopedPtr<'guard, List>),
    Nil,
    Number(isize),
    NumberObject(ScopedPtr<'guard, NumberObject>),
    Pair(ScopedPtr<'guard, Pair>),
    Partial(ScopedPtr<'guard, Partial>),
    Symbol(ScopedPtr<'guard, Symbol>),
    Text(ScopedPtr<'guard, Text>),
    Upvalue(ScopedPtr<'guard, Upvalue>),
}