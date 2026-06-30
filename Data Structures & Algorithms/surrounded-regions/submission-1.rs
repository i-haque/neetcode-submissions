impl Solution {
    pub fn solve(board: &mut Vec<Vec<char>>) {
        let (m, n) = (board.len(), board[0].len());
        let mut q: VecDeque<(usize, usize)> = VecDeque::new();
        let mut vis = vec![vec![false; n]; m];

        // collect all the boundary cells
        for j in 0..n {
            if board[0][j] == 'O' {
                q.push_back((0, j));
                vis[0][j] = true;
            }
            if board[m-1][j] == 'O' {
                q.push_back((m-1, j));
                vis[m-1][j] = true;
            }
        }

        for i in 0..m {
            if board[i][0] == 'O' {
                q.push_back((i, 0));
                vis[i][0] = true;
            }
            if board[i][n-1] == 'O' {
                q.push_back((i, n-1));
                vis[i][n-1] = true;
            }
        }

        // visit all the cells reachable from boundary
        while !q.is_empty() {
            for _ in 0..q.len() {
                let (i, j) = q.pop_front().unwrap();

                for (r, c) in [(i-1, j), (i+1, j), (i, j-1), (i, j+1)] {
                    if 0 <= r && r < m && 0 <= c && c < n {
                        if board[r][c] == 'O' && !vis[r][c] {
                            q.push_back((r, c));
                            vis[r][c] = true;
                        }
                    }
                }
            }
        }

        // mark visited cells as 'O' and unvisited cells as 'X'
        for i in 0..m {
            for j in 0..n {
                if vis[i][j] {
                    board[i][j] = 'O';
                } else {
                    board[i][j] = 'X';
                }
            }
        }
    }
}
