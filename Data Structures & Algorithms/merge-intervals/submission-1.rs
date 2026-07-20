impl Solution {
    pub fn merge(mut intervals: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        intervals.sort_unstable();
        let mut merged: Vec<Vec<i32>> = vec![];

        let (mut start, mut end) = (intervals[0][0], intervals[0][1]);
        for i in 1..intervals.len() {
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
