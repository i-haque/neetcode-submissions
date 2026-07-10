class DisjointSet:
    def __init__(self, n):
        self.parent = [i for i in range(n)]
        self.size = [1] * n
    
    def find_parent(self, node) -> int:
        if self.parent[node] == node:
            return node
        
        self.parent[node] = self.find_parent(self.parent[node])
        return self.parent[node]
    
    def union(self, u, v) -> bool:
        upu = self.find_parent(u)
        upv = self.find_parent(v)

        if upu == upv:
            return True
        
        if self.size[upu] < self.size[upv]:
            self.parent[upv] = upu
            self.size[upu] += self.size[upv]
        else:
            self.parent[upu] = upv
            self.size[upv] += self.size[upu]
        
        return False

class Solution:
    def validTree(self, n: int, edges: List[List[int]]) -> bool:
        ds = DisjointSet(n)
        for [u, v] in edges:
            if ds.union(u, v):
                return False
        
        connected_components = set()
        for i in range(n):
            connected_components.add(ds.find_parent(i))
        
        if len(connected_components) > 1:
            return False
        
        return True
        