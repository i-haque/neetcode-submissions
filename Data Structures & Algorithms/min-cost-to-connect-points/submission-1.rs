use std::collections::{BinaryHeap, HashSet};
use std::cmp::Reverse;

impl Solution {
    pub fn min_cost_connect_points(points: Vec<Vec<i32>>) -> i32 {
        let n = points.len();
        let mut g: Vec<Vec<(usize, i32)>> = vec![vec![]; n];
        for i in 0..n {
            let (x1, y1) = (points[i][0], points[i][1]);
            for j in i+1..n {
                let (x2, y2) = (points[j][0], points[j][1]);
                let dist = (x1 - x2).abs() + (y1 - y2).abs();
                g[i].push((j, dist));
                g[j].push((i, dist));
            }
        }

        let mut min_cost = 0;
        let mut vis: HashSet<usize> = HashSet::new();

        let mut h: BinaryHeap<Reverse<(i32, usize)>> = BinaryHeap::new();
        h.push(Reverse((0, 0)));
        while vis.len() < n {
            let Reverse((cost, node)) = h.pop().unwrap();
            if vis.contains(&node) {
                continue;
            }

            vis.insert(node);
            min_cost += cost;

            for edge in &g[node] {
                let (adj, c) = (edge.0, edge.1);
                if !vis.contains(&adj) {
                    h.push(Reverse((c, adj)));
                }
            }
        }

        min_cost
    }
}
