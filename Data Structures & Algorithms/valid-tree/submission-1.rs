use std::collections::HashSet;

struct DisjointSet {
    parent: Vec<usize>,
    size: Vec<i32>
}

impl DisjointSet {
    pub fn new(n: usize) -> Self {
        let mut parent = Vec::with_capacity(n);
        let mut size = Vec::with_capacity(n);
        for i in 0..n {
            parent.push(i);
            size.push(1);
        }

        Self { parent, size }
    }

    pub fn find_parent(&mut self, node: usize) -> usize {
        if self.parent[node] == node {
            return node;
        }

        self.parent[node] = self.find_parent(self.parent[node]);
        self.parent[node]
    }

    pub fn union(&mut self, u: usize, v: usize) -> bool {
        let upu = self.find_parent(u);
        let upv = self.find_parent(v);

        if upu == upv {
            return true;
        }

        if self.size[upu] < self.size[upv] {
            self.parent[upv] = upu;
            self.size[upu] = self.size[upv];
        } else {
            self.parent[upu] = upv;
            self.size[upv] += self.size[upu];
        }

        false
    }
}

impl Solution {
    pub fn valid_tree(n: i32, edges: Vec<Vec<i32>>) -> bool {
        let mut ds = DisjointSet::new(n as usize);
        for edge in edges {
            if ds.union(edge[0] as usize, edge[1] as usize) {
                return false;
            }
        }

        let mut connected_components: HashSet<usize> = HashSet::new();
        for i in 0..n as usize {
            connected_components.insert(ds.find_parent(i as usize));
        }

        if connected_components.len() > 1 {
            return false;
        }

        true
    }
}
