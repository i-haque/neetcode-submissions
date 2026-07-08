use std::collections::VecDeque;

impl Solution {
    pub fn find_order(num_courses: i32, prerequisites: Vec<Vec<i32>>) -> Vec<i32> {
        // topological sort
        let num_courses = num_courses as usize;

        // reverse graph (in this case it's already reversed)
        let mut g: Vec<Vec<i32>> = vec![vec![]; num_courses];
        for edge in prerequisites {
            g[edge[0] as usize].push(edge[1]);
        }

        // calculate indegree of each node
        let mut indegree: Vec<i32> = vec![0; num_courses];
        for neighbors in &g {
            for neighbor in neighbors {
                indegree[*neighbor as usize] += 1;
            }
        }

        // collect nodes with indegree 0, relax them and repeat using BFS
        let mut q: VecDeque<i32> = VecDeque::new();
        for (node, degree) in indegree.iter().enumerate() {
            if *degree == 0 {
                q.push_back(node as i32);
            }
        }

        let mut course_order: Vec<i32> = Vec::with_capacity(num_courses);
        while let Some(node) = q.pop_front() {
            course_order.push(node);

            for neighbor in &g[node as usize] {
                    indegree[*neighbor as usize] -= 1;
                    if indegree[*neighbor as usize] == 0 {
                        q.push_back(*neighbor);
                    }
            }
        }

        // if all nodes reached return reversed order or else return empty list
        if course_order.len() == num_courses {
            course_order.reverse();
            return course_order;
        }

        vec![]
    }
}
