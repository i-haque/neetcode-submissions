from heapq import heappush, heappop

class Solution:
    def minCostConnectPoints(self, points: List[List[int]]) -> int:
        n = len(points)

        g = [[] for _ in range(n)]
        for i in range(n):
            [x1, y1] = points[i]
            for j in range(i+1, n):
                [x2, y2] = points[j]
                dist = abs(x1 - x2) + abs(y1 - y2)
                g[i].append((j, dist))
                g[j].append((i, dist))
        
        vis = set()
        min_cost = 0

        h = [(0, 0)]    # (cost, node)
        while len(vis) < n:
            cost, node = heappop(h)
            if node in vis:
                continue
            
            vis.add(node)
            min_cost += cost
            
            for adj, c in g[node]:
                if adj not in vis:
                    heappush(h, (c, adj))
        
        return min_cost