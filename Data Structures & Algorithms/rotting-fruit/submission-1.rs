use std::collections::VecDeque;

impl Solution {
    pub fn oranges_rotting(mut grid: Vec<Vec<i32>>) -> i32 {
        let (m, n) = (grid.len(), grid[0].len());
        let mut q: VecDeque<(usize, usize)> = VecDeque::new();
        let mut fresh_oranges = 0;

        // count fresh oranges and store cells with rotten oranges
        for i in 0..m {
            for j in 0..n {
                if grid[i][j] == 1 {
                    fresh_oranges += 1;
                } else if grid[i][j] == 2 {
                    q.push_back((i, j));
                }
            }
        }

        // if no fresh oranges -> early exit
        if fresh_oranges == 0 {
            return 0;
        }

        // count the time takes to rot all fresh oranges using BFS
        let mut minutes = 0;
        while !q.is_empty() {
            for _ in 0..q.len() {
                let (i, j) = q.pop_front().unwrap();

                for (r, c) in [(i-1, j), (i+1, j), (i, j-1), (i, j+1)] {
                    if 0 <= r && r < m && 0 <= c && c < n && grid[r][c] == 1 {
                        grid[r][c] = 2;
                        fresh_oranges -= 1;
                        q.push_back((r, c));
                    }
                }
            }

            minutes += 1;
        }

        if fresh_oranges > 0 {
            return -1;
        }

        minutes - 1
    }
}
