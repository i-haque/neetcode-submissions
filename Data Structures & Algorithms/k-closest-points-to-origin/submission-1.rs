use std::collections::BinaryHeap;

impl Solution {
    pub fn k_closest(points: Vec<Vec<i32>>, k: i32) -> Vec<Vec<i32>> {
        let mut max_heap: BinaryHeap<(i32, Vec<i32>)> = BinaryHeap::new();

        for point in points {
            let dist = (point[0] * point[0]) + (point[1] * point[1]);
            max_heap.push((dist, vec![point[0], point[1]]));

            if max_heap.len() > k as usize {
                max_heap.pop();
            }
        }

        max_heap.into_iter().map(|item| item.1).collect::<Vec<Vec<i32>>>()
    }
}
