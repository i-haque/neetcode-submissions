use std::collections::VecDeque;

impl Solution {
    pub fn max_sliding_window(nums: Vec<i32>, k: i32) -> Vec<i32> {
        let mut res = Vec::with_capacity(nums.len() - (k-1) as usize);
        let mut q: VecDeque<(i32, usize)> = VecDeque::new();

        for (i, num) in nums.iter().enumerate() {
            while !q.is_empty() && q.back().unwrap().0 < *num {
                q.pop_back();
            }
            q.push_back((*num, i));

            if (i as i32) >= k-1 {
                res.push(q[0].0);
            }

            if (i as i32) - (q[0].1 as i32) == k-1 {
                q.pop_front();
            }
        }

        res
    }
}
