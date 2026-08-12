from heapq import heappush, heappop

class Solution:
    def findCheapestPrice(self, n: int, flights: List[List[int]], src: int, dst: int, k: int) -> int:
        g = [[] for _ in range(n)]
        for [u, v, p] in flights:
            g[u].append((v, p))
        
        prices = [float('inf')] * n
        prices[src] = 0
        
        h = [(src, prices[src], 0)]
        while h:
            node, price, stops = heappop(h)
            if stops <= k:
                for (adj, p) in g[node]:
                    if price + p < prices[adj]:
                        prices[adj] = price + p
                        heappush(h, (adj, price + p, stops + 1))
        
        return prices[dst] if prices[dst] != float('inf') else -1
            