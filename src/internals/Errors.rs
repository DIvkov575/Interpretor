use std::fmt;
use thiserror::Error;
use crate::frontend::Token::SourcePos;
use crate::internals::Block::BlockError;

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum AllocError {
    /// Some attribute of the allocation, most likely the size requested,
    /// could not be fulfilled
    BadRequest,
    /// Out of memory - allocating the space failed
    OOM,
}

#[derive(Debug, PartialEq)]
pub struct RuntimeError {
    kind: ErrorKind,
    pos: Option<SourcePos>,
}

impl RuntimeError {
    pub fn new(kind: ErrorKind) -> RuntimeError {
        RuntimeError {
            kind: kind,
            pos: None,
        }
    }

    pub fn with_pos(kind: ErrorKind, pos: SourcePos) -> RuntimeError {
        RuntimeError {
            kind: kind,
            pos: Some(pos),
        }
    }

    pub fn error_kind(&self) -> &ErrorKind {
        &self.kind
    }

    pub fn error_pos(&self) -> Option<SourcePos> {
        self.pos
    }

    /// Given the relevant source code string, show the error in context
    pub fn print_with_source(&self, source: &str) {
        if let Some(ref pos) = self.pos {
            let mut iter = source.lines().enumerate();

            while let Some((count, line)) = iter.next() {
                // count starts at 0, line numbers start at 1
                if count + 1 == pos.line as usize {
                    println!("error: {}", self);
                    println!("{:5}|{}", pos.line, line);
                    println!("{:5}|{:width$}^", " ", " ", width = pos.column as usize);
                    println!("{:5}|", " ");
                    return;
                }
            }
        } else {
            println!("error: {}", self);
        }
    }
}

impl fmt::Display for RuntimeError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self.kind {
            ErrorKind::IOError(ref reason) => write!(f, "IO Error: {}", reason),
            ErrorKind::LexerError(ref reason) => write!(f, "Parse error: {}", reason),
            ErrorKind::ParseError(ref reason) => write!(f, "Parse error: {}", reason),
            ErrorKind::EvalError(ref reason) => write!(f, "Evaluation error: {}", reason),
            ErrorKind::OutOfMemory => write!(f, "Out of memory!"),
            ErrorKind::BadAllocationRequest => {
                write!(f, "An invalid memory size allocation was requested!")
            }
            ErrorKind::BoundsError => write!(f, "Indexing bounds error"),
            ErrorKind::KeyError => write!(f, "Key does not exist in Dict"),
            ErrorKind::UnhashableError => write!(f, "Attempt to access Dict with unhashable key"),
            ErrorKind::MutableBorrowError => write!(
                f,
                "Attempt to modify a container that is already mutably borrowed"
            ),
        }
    }
}

/// Convert from io::Error

/// Convert from BlockError
impl From<BlockError> for RuntimeError {
    fn from(other: BlockError) -> RuntimeError {
        match other {
            BlockError::OOM => RuntimeError::new(ErrorKind::OutOfMemory),
            BlockError::BadRequest => RuntimeError::new(ErrorKind::BadAllocationRequest),
        }
    }
}

/// Convert from AllocError
impl From<AllocError> for RuntimeError {
    fn from(other: AllocError) -> RuntimeError {
        match other {
            AllocError::OOM => RuntimeError::new(ErrorKind::OutOfMemory),
            AllocError::BadRequest => RuntimeError::new(ErrorKind::BadAllocationRequest),
        }
    }
}


#[derive(Debug, PartialEq)]
pub enum ErrorKind {
    IOError(String),
    LexerError(String),
    ParseError(String),
    EvalError(String),
    BadAllocationRequest,
    OutOfMemory,
    BoundsError,
    KeyError,
    UnhashableError,
    MutableBorrowError,

}
