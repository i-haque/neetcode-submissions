use std::collections::VecDeque;

impl Solution {
    pub fn num_islands(grid: Vec<Vec<char>>) -> i32 {
        let (m, n) = (grid.len(), grid[0].len());
        let mut vis = vec![vec![false; n]; m];

        let mut islands = 0;

        for i in 0..m {
            for j in 0..n {
                if grid[i][j] == '1' && !vis[i][j] {
                    let mut q: VecDeque<(usize, usize)> = VecDeque::from([(i, j)]);
                    vis[i][j] = true;

                    while !q.is_empty() {
                        for _ in 0..q.len() {
                            let (r, c) = q.pop_front().unwrap();
                            
                            for (row, col) in [(r + 1, c), (r - 1, c), (r, c + 1), (r, c - 1)] {
                                if 0 <= row && row < m && 0 <= col && col < n {
                                    if grid[row][col] == '1' && !vis[row][col] {
                                        q.push_back((row, col));
                                        vis[row][col] = true;
                                    }
                                }
                            }
                        }
                    }

                    islands += 1
                }
            }
        }

        islands
    }
}
