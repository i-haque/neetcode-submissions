use std::cmp::min;

impl Solution {
    pub fn erase_overlap_intervals(mut intervals: Vec<Vec<i32>>) -> i32 {
        intervals.sort_unstable();
        let mut removals = 0;
        let (mut prev_start, mut prev_end) = (intervals[0][0], intervals[0][1]);

        for i in 1..intervals.len() {
            let (curr_start, curr_end) = (intervals[i][0], intervals[i][1]);
            if prev_end > curr_start {
                prev_end = min(prev_end, curr_end);
                removals += 1;
            } else {
                (prev_start, prev_end) = (curr_start, curr_end);
            }
        }

        removals
    }
}
