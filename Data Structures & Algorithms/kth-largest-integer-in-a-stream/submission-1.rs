use std::cmp::Reverse;
use std::collections::BinaryHeap;

struct KthLargest {
    size: usize,
    min_heap: BinaryHeap<Reverse<i32>>
}

impl KthLargest {
    pub fn new(k: i32, nums: Vec<i32>) -> Self {
        let mut min_heap: BinaryHeap<Reverse<i32>> = BinaryHeap::with_capacity((k + 1) as usize);
        for num in nums {
            min_heap.push(Reverse(num));
            if min_heap.len() > k as usize {
                min_heap.pop();
            }
        }

        Self { size: k as usize, min_heap}
    }

    pub fn add(&mut self, val: i32) -> i32 {
        self.min_heap.push(Reverse(val));
        if self.min_heap.len() > self.size {
            self.min_heap.pop();
        }

        self.min_heap.peek().unwrap().0
    }
}
