use std::collections::{HashMap, HashSet};

impl Solution {
    pub fn is_valid_sudoku(board: Vec<Vec<char>>) -> bool {
        let mut rows: HashMap<usize, HashSet<char>> = HashMap::with_capacity(9);
        let mut cols: HashMap<usize, HashSet<char>> = HashMap::with_capacity(9);
        let mut sq: HashMap<(usize, usize), HashSet<char>> = HashMap::with_capacity(9);

        for i in 0..9 {
            for j in 0..9 {
                let val: char = board[i][j];
                if val != '.' {
                    let in_row: bool = !rows.entry(i).or_default().insert(val);
                    let in_col: bool = !cols.entry(j).or_default().insert(val);
                    let in_sq: bool = !sq.entry((i/3, j/3)).or_default().insert(val);
                    if in_row || in_col || in_sq {
                        return false;
                    }
                }
            }
        }
        true
    }
}
