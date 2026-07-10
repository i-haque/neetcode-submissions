class DisjointSet:
    def __init__(self, n):
        self.parent = [i for i in range(n)]
        self.size = [1] * n
    
    def find_parent(self, node) -> int:
        if self.parent[node] == node:
            return node
        
        self.parent[node] = self.find_parent(self.parent[node])
        return self.parent[node]
    
    def union(self, u, v) -> None:
        upu = self.find_parent(u)
        upv = self.find_parent(v)

        if upu != upv:
            if self.size[upu] < self.size[upv]:
                self.parent[upv] = upu
                self.size[upu] += self.size[upv]
            else:
                self.parent[upu] = upv
                self.size[upv] += self.size[upu]

class Solution:
    def countComponents(self, n: int, edges: List[List[int]]) -> int:
        ds = DisjointSet(n)
        for [u, v] in edges:
            ds.union(u, v)
        
        connected_components = set()
        for i in range(n):
            connected_components.add(ds.find_parent(i))
        
        return len(connected_components)