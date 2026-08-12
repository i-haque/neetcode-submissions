use std::collections::VecDeque;

impl Solution {
    pub fn find_cheapest_price(n: i32, flights: Vec<Vec<i32>>, src: i32, dst: i32, k: i32) -> i32 {
        let mut g: Vec<Vec<(i32, i32)>> = vec![vec![]; n as usize];
        for flight in flights {
            g[flight[0] as usize].push((flight[1], flight[2]));
        }

        let mut prices: Vec<i32> = vec![i32::MAX; n as usize];
        prices[src as usize] = 0;

        // (node, curr_price)
        let mut q: VecDeque<(i32, i32)> = VecDeque::new();
        q.push_back((src, 0));
        let mut stops = 0;

        while !q.is_empty() && stops <= k {
            for _ in 0..q.len() {
                let (node, curr_price) = q.pop_front().unwrap();

                for item in &g[node as usize] {
                    let (adj, price) = (item.0, item.1);
                    if curr_price + price < prices[adj as usize] {
                        prices[adj as usize] = curr_price + price;
                        q.push_back((adj, prices[adj as usize]));
                    }
                }
            }
            
            stops += 1;
        }

        if prices[dst as usize] == i32::MAX {
            return -1;
        }
        prices[dst as usize]
    }
}
