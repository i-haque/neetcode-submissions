use std::collections::{HashMap, BinaryHeap};
use std::cmp::Reverse;

impl Solution {
    pub fn top_k_frequent(nums: Vec<i32>, k: i32) -> Vec<i32> {
        let mut freq: HashMap<i32, i32> = HashMap::new();
        for num in nums {
            freq.entry(num).and_modify(|val| *val += 1).or_insert(1);
        }

        let mut heap: BinaryHeap<Reverse<(i32, i32)>> = BinaryHeap::new();
        for (key, val) in freq {
            heap.push(Reverse((val, key)));
            if heap.len() > k as usize {
                heap.pop();
            }
        }
        let mut res: Vec<i32> = Vec::with_capacity(k as usize);
        while let Some(Reverse((_, key))) = heap.pop() {
            res.push(key)
        }
        res
    }
}
