use std::cell::UnsafeCell;
use std::marker::PhantomData;
use std::mem::replace;
use crate::internals::Alloc::AllocError;
use crate::internals::BumpBlock::BumpBlock;

pub struct BlockList {
    pub head: Option<BumpBlock>,
    pub overflow: Option<BumpBlock>,
    pub rest: Vec<BumpBlock>,
}
impl BlockList {
    pub(crate) fn overflow_alloc(&mut self, alloc_size: usize) -> Result<*const u8, AllocError> {
        match self.overflow {
            Some(ref mut overflow) => {
                // This is a medium object that might fit in the current block...
                match overflow.inner_alloc(alloc_size) {
                    // the block has a suitable hole
                    Some(space) => Ok(space),
                    None => {
                        let previous = replace(overflow, BumpBlock::new()?);

                        self.rest.push(previous);

                        overflow.inner_alloc(alloc_size).expect("Unexpected error!")
                    }
                }
            },
            None => {
                let mut overflow = BumpBlock::new()?;

                // object size < block size means we can't fail this expect
                let space = overflow
                    .inner_alloc(alloc_size)
                    .expect("We expected this object to fit!");

                self.overflow = Some(overflow);

                space
            }
        }    }
}

