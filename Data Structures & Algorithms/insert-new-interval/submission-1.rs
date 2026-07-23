use std::cmp::{max, min};

impl Solution {
    pub fn insert(mut intervals: Vec<Vec<i32>>, new_interval: Vec<i32>) -> Vec<Vec<i32>> {
        intervals.push(new_interval);
        intervals.sort_unstable();
        let n = intervals.len();

        let mut merged: Vec<Vec<i32>> = vec![];
        let (mut start, mut end) = (intervals[0][0], intervals[0][1]);

        for i in 1..n {
            let (curr_start, curr_end) = (intervals[i][0], intervals[i][1]);

            if end >= curr_start {
                start = min(start, curr_start);
                end = max(end, curr_end);
            } else {
                merged.push(vec![start, end]);
                (start, end) = (curr_start, curr_end);
            }
        }

        merged.push(vec![start, end]);
        merged
    }
}
