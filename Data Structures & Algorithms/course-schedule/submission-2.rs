impl Solution {
    pub fn can_finish(num_courses: i32, prerequisites: Vec<Vec<i32>>) -> bool {
        // topological sort

        // reverse the graph (here the graph is already in reverse order)
        let mut g: Vec<Vec<usize>> = vec![vec![]; num_courses as usize];
        for edge in &prerequisites {
            g[edge[0] as usize].push(edge[1] as usize);
        }

        // calculate the indegree of each node
        let mut indegree = vec![0; num_courses as usize];
        for (node, neighbors) in g.iter().enumerate() {
            for neighbor in neighbors {
                indegree[*neighbor] += 1;
            }
        }

        // store all the nodes with indegree 0
        let mut q: VecDeque<usize> = VecDeque::new();
        for (node, degree) in indegree.iter().enumerate() {
            if *degree == 0 {
                q.push_back(node);
            }
        }

        // using BFS, break connections for each node and store all the nodes with indegree 0
        while let Some(node) = q.pop_front() {
            for neighbor in &g[node] {
                indegree[*neighbor] -= 1;

                if indegree[*neighbor] == 0 {
                    q.push_back(*neighbor);
                }
            }
        }

        // return false if any node has indegree > 0
        for degree in indegree {
            if degree > 0 {
                return false;
            }
        }

        true
    }
}
