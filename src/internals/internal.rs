use std::alloc::{alloc, Layout};
use std::ptr::NonNull;
use crate::internals::Block::{BlockError, BlockPtr, BlockSize};

pub fn alloc_block(size: BlockSize) -> Result<BlockPtr, BlockError> {
    unsafe {
        let layout = Layout::from_size_align_unchecked(size, size);

        let ptr = alloc(layout);
        if ptr.is_null() {
            Err(BlockError::OOM)
        } else {
            Ok(NonNull::new_unchecked(ptr))
        }
    }
}