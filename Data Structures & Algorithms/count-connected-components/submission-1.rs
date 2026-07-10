impl Solution {
    pub fn count_components(n: i32, edges: Vec<Vec<i32>>) -> i32 {
        let mut g: Vec<Vec<i32>> = vec![vec![]; n as usize];
        for edge in edges {
            g[edge[0] as usize].push(edge[1]);
            g[edge[1] as usize].push(edge[0]);
        }

        let mut connected_components: i32 = 0;

        let mut vis: Vec<bool> = vec![false; n as usize];
        for i in 0..n {
            if !vis[i as usize] {
                Self::dfs(&g, i, &mut vis);
                connected_components += 1;
            }
        }

        connected_components
    }

    pub fn dfs(g: &Vec<Vec<i32>>, node: i32, vis: &mut Vec<bool>) {
        vis[node as usize] = true;

        for neighbor in &g[node as usize] {
            if !vis[*neighbor as usize] {
                Self::dfs(g, *neighbor, vis);
            }
        }
    }
}
