use std::{cmp::Reverse, collections::BinaryHeap};
impl Solution {
    pub fn min_interval(mut intervals: Vec<Vec<i32>>, queries: Vec<i32>) -> Vec<i32> {
        let mut queries: Vec<(i32, usize)> = queries
            .into_iter()
            .enumerate()
            .map(|(i, q)| (q, i))
            .collect();
        queries.sort_unstable();
        intervals.sort_unstable();

        let mut res = vec![-1; queries.len()];
        let mut h: BinaryHeap<Reverse<(i32, i32)>> = BinaryHeap::new();
        let mut j = 0;

        for query in queries {
            let (q, i) = (query.0, query.1);

            while j < intervals.len() && intervals[j][0] <= q {
                let (start, end) = (intervals[j][0], intervals[j][1]);
                h.push(Reverse((end - start + 1, end)));
                j += 1
            }

            while !h.is_empty() && h.peek().unwrap().0.1 < q {
                h.pop();
            }

            if !h.is_empty() {
                res[i] = h.peek().unwrap().0.0;
            }
        }

        res
    }
}
