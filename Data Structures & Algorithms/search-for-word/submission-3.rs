impl Solution {
    pub fn exist(board: Vec<Vec<char>>, word: String) -> bool {
        let (m, n) = (board.len(), board[0].len());
        let word: Vec<char> = word.chars().collect();
        let mut vis: Vec<Vec<bool>> = vec![vec![false; n]; m];

        for i in 0..m {
            for j in 0..n {
                if board[i][j] == word[0] {
                    if Self::dfs(&board, i, j, &word, 0, &mut vis) {
                        return true;
                    }
                }
            }
        }

        false
    }

    pub fn dfs(board: &Vec<Vec<char>>, row: usize, col: usize, word: &[char], index: usize, vis: &mut Vec<Vec<bool>>) -> bool {
        if index == word.len() - 1 {
            return true;
        }

        vis[row][col] = true;

        for (r, c) in [(row - 1, col), (row + 1, col), (row, col - 1), (row, col + 1)] {
            if 0 <= r && r < board.len() && 0 <= c && c < board[0].len() && !vis[r][c] {
                if index + 1 < word.len() && word[index + 1] == board[r][c] {
                    if Self::dfs(board, r, c, word, index + 1, vis) {
                        return true;
                    }
                }
            }
        }

        vis[row][col] = false;

        false
    }
}
