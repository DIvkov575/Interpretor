use std::cell::Cell;
use std::fmt;
use std::future::join;
use std::io::Bytes;
use crate::evalrus::MutatorView::MutatorView;
use crate::evalrus::Ptrs::{CellPtr, ScopedPtr, TaggedPtr};
use crate::evalrus::Traits::MutatorScope;
use crate::frontend::Array::{Array, ArraySize, List};
use crate::frontend::Traits::{IndexedContainer, StackContainer};
use crate::internals::Errors::RuntimeError;

type Register = u8;
type LiteralInteger = i16;
pub type ArrayOpcode = Array<Opcode>;

pub type LiteralId = u16;

pub type Literals = List;
pub type UpvalueId = u8;

/// An instruction jump target is a signed integer, relative to the jump instruction
pub type JumpOffset = i16;
/// Jump offset when the target is still unknown.
pub const JUMP_UNKNOWN: i16 = 0x7fff;


#[derive(Clone)]
pub struct ByteCode {
    code: ArrayOpcode,
    literals: Literals,
}

impl ByteCode {
    /// Instantiate a blank ByteCode instance
    pub fn alloc<'guard>(
        mem: &'guard MutatorView,
    ) -> Result<ScopedPtr<'guard, ByteCode>, RuntimeError> {
        mem.alloc(ByteCode {
            code: ArrayOpcode::new(),
            literals: Literals::new(),
        })
    }

    /// Append an instuction to the back of the sequence
    pub fn push<'guard>(&self, mem: &'guard MutatorView, op: Opcode) -> Result<(), RuntimeError> {
        self.code.push(mem, op)
    }

    /// Set the jump offset of an existing jump instruction to a new value
    pub fn update_jump_offset<'guard>(
        &self,
        mem: &'guard MutatorView,
        instruction: ArraySize,
        offset: JumpOffset,
    ) -> Result<(), RuntimeError> {
        let code = self.code.get(mem, instruction)?;
        let new_code = match code {
            Opcode::Jump { offset: _ } => Opcode::Jump { offset },
            Opcode::JumpIfTrue { test, offset: _ } => Opcode::JumpIfTrue { test, offset },
            Opcode::JumpIfNotTrue { test, offset: _ } => Opcode::JumpIfNotTrue { test, offset },
            _ => {
                return Err(err_eval(
                    "Cannot modify jump offset for non-jump instruction",
                ))
            }
        };
        self.code.set(mem, instruction, new_code)?;
        Ok(())
    }

    /// Append a literal-load operation to the back of the sequence
    pub fn push_loadlit<'guard>(
        &self,
        mem: &'guard MutatorView,
        dest: Register,
        literal_id: LiteralId,
    ) -> Result<(), RuntimeError> {
        // TODO clone anything mutable
        self.code
            .push(mem, Opcode::LoadLiteral { dest, literal_id })
    }

    /// Push a literal pointer/value to the back of the literals list and return it's index
    pub fn push_lit<'guard>(
        &self,
        mem: &'guard MutatorView,
        literal: TaggedScopedPtr<'guard>,
    ) -> Result<LiteralId, RuntimeError> {
        let lit_id = self.literals.length() as u16;
        StackAnyContainer::push(&self.literals, mem, literal)?;
        Ok(lit_id)
    }

    /// Get the index into the bytecode array of the last instruction
    pub fn last_instruction(&self) -> ArraySize {
        self.code.length() - 1
    }

    /// Get the index into the bytecode array of the next instruction that will be pushed
    pub fn next_instruction(&self) -> ArraySize {
        self.code.length()
    }
}

// impl printer::Print for ByteCode {
//     fn print<'guard>(
//         &self,
//         guard: &'guard dyn MutatorScope,
//         f: &mut fmt::Formatter,
//     ) -> fmt::Result {
//         let mut instr_str = String::new();
//
//         self.code.access_slice(guard, |code| {
//             instr_str = itertools::join(code.iter().map(|opcode| format!("{:?}", opcode)), "\n")
//         });
//
//         write!(f, "{}", instr_str)
//     }
// }

/// An InstructionStream is a pointer to a ByteCode instance and an instruction pointer giving the
/// current index into the ByteCode
// ANCHOR: DefInstructionStream
pub struct InstructionStream {
    instructions: CellPtr<ByteCode>,
    ip: Cell<ArraySize>,
}
// ANCHOR_END: DefInstructionStream

impl InstructionStream {
    /// Create an InstructionStream instance with the given ByteCode instance that will be iterated over
    pub fn alloc<'guard>(
        mem: &'guard MutatorView,
        code: ScopedPtr<'_, ByteCode>,
    ) -> Result<ScopedPtr<'guard, InstructionStream>, RuntimeError> {
        mem.alloc(InstructionStream {
            instructions: CellPtr::new_with(code),
            ip: Cell::new(0),
        })
    }

    /// Change to a different stack frame, either as a function call or a return
    // ANCHOR: DefInstructionStreamSwitchFrame
    pub fn switch_frame(&self, code: ScopedPtr<'_, ByteCode>, ip: ArraySize) {
        self.instructions.set(code);
        self.ip.set(ip);
    }
    // ANCHOR_END: DefInstructionStreamSwitchFrame

    /// Retrieve the next instruction and return it, incrementing the instruction pointer
    // TODO: https://github.com/rust-hosted-langs/book/issues/39
    // ANCHOR: DefInstructionStreamGetNextOpcode
    pub fn get_next_opcode<'guard>(
        &self,
        guard: &'guard dyn MutatorScope,
    ) -> Result<Opcode, RuntimeError> {
        let instr = self
            .instructions
            .get(guard)
            .code
            .get(guard, self.ip.get())?;
        self.ip.set(self.ip.get() + 1);
        Ok(instr)
    }
    // ANCHOR_END: DefInstructionStreamGetNextOpcode

    /// Given an index into the literals list, return the pointer in the list at that index.
    pub fn get_literal<'guard>(
        &self,
        guard: &'guard dyn MutatorScope,
        lit_id: LiteralId,
    ) -> Result<TaggedPtr, RuntimeError> {
        Ok(IndexedContainer::get(
            &self.instructions.get(guard).literals,
            guard,
            lit_id as ArraySize,
        )?
            .get_ptr())
    }

    /// Return the next instruction pointer
    pub fn get_next_ip(&self) -> ArraySize {
        self.ip.get()
    }

    /// Adjust the instruction pointer by the given signed offset from the current ip
    pub fn jump(&self, offset: JumpOffset) {
        let mut ip = self.ip.get() as i32;
        ip += offset as i32;
        self.ip.set(ip as ArraySize);
    }
}


#[derive(Copy, Clone)]
pub enum Opcode {
    Add {
        dest: Register,
        a: Register,
        b: Register
    },
    LoadLiteral {
        dest: Register,
        value: LiteralInteger
    }
}
