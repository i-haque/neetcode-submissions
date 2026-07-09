use std::collections::HashSet;

impl Solution {
    pub fn pacific_atlantic(heights: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let (m, n) = (heights.len(), heights[0].len());
        let mut pac: HashSet<(usize, usize)> = HashSet::new();
        let mut atl: HashSet<(usize, usize)> = HashSet::new();

        // cells reachable from pacific co-ordinates
        for c in 0..n {
            Self::dfs(&heights, 0, c, &mut pac);
        }

        for r in 0..m {
            Self::dfs(&heights, r, 0, &mut pac);
        }

        // cells reachable from atlantic co-ordinates
        for c in 0..n {
            Self::dfs(&heights, m-1, c, &mut atl);
        }

        for r in 0..m {
            Self::dfs(&heights, r, n-1, &mut atl);
        }

        let mut res: Vec<Vec<i32>> = vec![];
        for (r, c) in pac {
            if atl.contains(&(r, c)) {
                res.push(vec![r as i32, c as i32]);
            }
        }

        res
    }


    pub fn dfs(heights: &Vec<Vec<i32>>, i: usize, j: usize, vis: &mut HashSet<(usize, usize)>) {
        vis.insert((i, j));

        for (r, c) in [(i-1, j), (i+1, j), (i, j-1), (i, j+1)] {
            if 0 <= r && r < heights.len() && 0 <= c && c < heights[0].len() {
                if !vis.contains(&(r, c)) && heights[r][c] >= heights[i][j] {
                    Self::dfs(heights, r, c, vis);
                }
            }
        }
    }
}
