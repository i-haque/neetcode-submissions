use std::collections::BinaryHeap;

impl Solution {
    pub fn last_stone_weight(stones: Vec<i32>) -> i32 {
        let mut max_heap: BinaryHeap<i32> = BinaryHeap::from(stones);

        while max_heap.len() > 1 {
            let x = max_heap.pop().unwrap();
            let y = max_heap.pop().unwrap();

            if x > y {
                max_heap.push(x - y);
            } else if y > x {
                max_heap.push(y - x);
            }
        }

        if max_heap.is_empty() {
            return 0;
        }

        max_heap.pop().unwrap()
    }
}
