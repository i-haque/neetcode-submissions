class DisjointSet:
    def __init__(self, n):
        self.parent = [i for i in range(n+1)]
        self.size = [1] * (n+1)

    def find_parent(self, node) -> int:
        if self.parent[node] == node:
            return node
        
        self.parent[node] = self.find_parent(self.parent[node])
        return self.parent[node]

    # return True if parents are already connected
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
    def findRedundantConnection(self, edges: List[List[int]]) -> List[int]:
        n = len(edges)
        ds = DisjointSet(n)

        for [u, v] in edges:
            if ds.union(u, v):
                return [u, v]

        return []