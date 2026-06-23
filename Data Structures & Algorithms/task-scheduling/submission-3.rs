use std::collections::{BinaryHeap, HashMap};
use std::cmp::Reverse;

impl Solution {
    pub fn least_interval(tasks: Vec<char>, n: i32) -> i32 {
        let mut freq: HashMap<char, i32> = HashMap::with_capacity(26);
        for task in tasks {
            *freq.entry(task).or_insert(0) += 1;
        }
        
        let mut min_heap: BinaryHeap<Reverse<(i32, i32)>> = BinaryHeap::with_capacity(26);
        for (_, val) in freq {
            min_heap.push(Reverse((1, val)));
        }

        let mut total_time = 0;
        while !min_heap.is_empty() {
            let (time, count) = min_heap.pop().unwrap().0;
            if total_time >= time {
                total_time += 1;
            } else {
                total_time = time;
            }

            if count - 1 > 0 {
                min_heap.push(Reverse((time + n + 1, count - 1)));
            }
        }

        total_time
    }
}
