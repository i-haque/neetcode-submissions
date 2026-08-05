use std::{cmp::{Reverse, max}, collections::BinaryHeap};

impl Solution {
    pub fn network_delay_time(times: Vec<Vec<i32>>, n: i32, k: i32) -> i32 {
        let mut g: Vec<Vec<(usize, i32)>> = vec![vec![]; (n+1) as usize];
        for time in times {
            g[time[0] as usize].push((time[1] as usize, time[2]));
        }

        let mut total_time = vec![i32::MAX; (n+1) as usize];
        total_time[0] = 0;
        total_time[k as usize] = 0;

        let mut min_heap: BinaryHeap<Reverse<(i32, usize)>> = BinaryHeap::new();
        min_heap.push(Reverse((0, k as usize)));
        while let Some(Reverse((time, node))) = min_heap.pop() {
            for item in &g[node] {
                let (neighbor, t) = (item.0, item.1);
                if time + t < total_time[neighbor] {
                    total_time[neighbor] = time + t;
                    min_heap.push(Reverse((time + t, neighbor)));
                }
            }
        }

        let mut res = 0;
        for time in total_time {
            if time == i32::MAX {
                return -1;
            } else {
                res = max(res, time);
            }
        }

        res
    }
}
