use std::cmp::Reverse;
use std::collections::BinaryHeap;

struct MedianFinder {
    max_heap: BinaryHeap<i32>,
    min_heap: BinaryHeap<Reverse<i32>>
}

impl MedianFinder {
    pub fn new() -> Self {
        Self {
            max_heap: BinaryHeap::new(),
            min_heap: BinaryHeap::new()
        }
    }

    pub fn add_num(&mut self, num: i32) {
        if !self.max_heap.is_empty() && *self.max_heap.peek().unwrap() < num {
            self.min_heap.push(Reverse(num));
        } else {
            self.max_heap.push(num);
        }

        // balance the heaps
        let (n1, n2) = (self.max_heap.len() as i32, self.min_heap.len() as i32);

        if n1 - n2 > 1 {
            let x = self.max_heap.pop().unwrap();
            self.min_heap.push(Reverse(x));
        } else if n2 - n1 > 1 {
            let Reverse(x) = self.min_heap.pop().unwrap();
            self.max_heap.push(x);
        }
    }

    pub fn find_median(&self) -> f64 {
        let (n1, n2) = (self.max_heap.len() as i32, self.min_heap.len() as i32);

        // bigger heap contains the median
        if n1 > n2 {
            *self.max_heap.peek().unwrap() as f64
        } else if n2 > n1 {
            self.min_heap.peek().unwrap().0 as f64
        } else {
            let m1 = *self.max_heap.peek().unwrap() as f64;
            let m2 = self.min_heap.peek().unwrap().0 as f64;
            (m1 + m2) / 2.0
        }
    }
}
