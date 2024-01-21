use crate::internals::constants;

pub struct BlockMeta {
    lines: *mut u8,
}

impl BlockMeta {

    /// When it comes to finding allocatable holes, we bump-allocate downward.
    pub fn find_next_available_hole(
        &self,
        starting_at: usize,
        alloc_size: usize,
    ) -> Option<(usize, usize)> {
        // The count of consecutive avaliable holes. Must take into account a conservatively marked
        // hole at the beginning of the sequence.
        let mut count = 0;
        let starting_line = starting_at / constants::LINE_SIZE;
        let lines_required = (alloc_size + constants::LINE_SIZE - 1) / constants::LINE_SIZE;
        // Counting down from the given search start index
        let mut end = starting_line;

        for index in (0..starting_line).rev() {
            let marked = unsafe { *self.lines.add(index) };

            if marked == 0 {
                // count unmarked lines
                count += 1;

                if index == 0 && count >= lines_required {
                    let limit = index * constants::LINE_SIZE;
                    let cursor = end * constants::LINE_SIZE;
                    return Some((cursor, limit));
                }
            } else {
                // This block is marked
                if count > lines_required {
                    // But at least 2 previous blocks were not marked. Return the hole, considering the
                    // immediately preceding block as conservatively marked
                    let limit = (index + 2) * constants::LINE_SIZE;
                    let cursor = end * constants::LINE_SIZE;
                    return Some((cursor, limit));
                }

                // If this line is marked and we didn't return a new cursor/limit pair by now,
                // reset the hole search state
                count = 0;
                end = index;
            }
        }

        None
    }}