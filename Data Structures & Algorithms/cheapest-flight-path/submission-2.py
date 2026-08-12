class Solution:
    def findCheapestPrice(self, n: int, flights: List[List[int]], src: int, dst: int, k: int) -> int:
        g = [[] for _ in range(n)]
        for [u, v, p] in flights:
            g[u].append((v, p))
        
        prices = [float('inf')] * n
        prices[src] = 0
        
        # (node, curr_price)
        q = deque([(src, 0)])
        stops = 0

        while q and stops <= k:
            for _ in range(len(q)):
                node, curr_price = q.popleft()
                for (adj, price) in g[node]:
                    if curr_price + price < prices[adj]:
                        prices[adj] = curr_price + price
                        q.append((adj, prices[adj]))

            stops += 1
        
        return -1 if prices[dst] == float('inf') else prices[dst]


            