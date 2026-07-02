use std::collections::VecDeque;

impl Solution {
    pub fn islands_and_treasure(grid: &mut Vec<Vec<i32>>) {
        let INF = 2147483647;
        let (m, n) = (grid.len(), grid[0].len());
        let mut q: VecDeque<(usize, usize, i32)> = VecDeque::new();

        for i in 0..m {
            for j in 0..n {
                if grid[i][j] == 0 {
                    q.push_back((i, j, 0));
                }
            }
        }

        while !q.is_empty() {
            for _ in 0..q.len() {
                let (i, j, dist) = q.pop_front().unwrap();

                for (r, c) in [(i-1, j), (i+1, j), (i, j-1), (i, j+1)] {
                    if 0 <= r && r < m && 0 <= c && c < n && grid[r][c] == INF {
                        grid[r][c] = dist + 1;
                        q.push_back((r, c, dist + 1));
                    }
                }
            }
        }
    }
}
