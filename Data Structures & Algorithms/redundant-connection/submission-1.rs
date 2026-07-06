struct DisjointSet {
    parent: Vec<usize>,
    size: Vec<u8>
}

impl DisjointSet {
    fn new(n: usize) -> Self {
        let parent = (0..(n+1)).into_iter().collect();
        let size = vec![1; n+1];
        Self { parent, size }
    }

    fn find_parent(&mut self, node: usize) -> usize {
        if self.parent[node] == node {
            return node;
        }

        self.parent[node] = self.find_parent(self.parent[node]);
        self.parent[node]
    }

    fn union(&mut self, u: usize, v: usize) -> bool {
        let upu = self.find_parent(u);
        let upv = self.find_parent(v);
        if upu == upv {
            return true;
        }

        if self.size[upu] < self.size[upv] {
            self.parent[upv] = upu;
            self.size[upu] += self.size[upv];
        } else {
            self.parent[upu] = upv;
            self.size[upv] += self.size[upu];
        }

        false
    }
}

impl Solution {
    pub fn find_redundant_connection(edges: Vec<Vec<i32>>) -> Vec<i32> {
        let n = edges.len();
        let mut ds = DisjointSet::new(n);

        for edge in edges {
            if ds.union(edge[0] as usize, edge[1] as usize) {
                return vec![edge[0], edge[1]];
            }
        }
        vec![]
    }
}
