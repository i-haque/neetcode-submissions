from heapq import heappush, heappop

class Solution:
    def networkDelayTime(self, times: List[List[int]], n: int, k: int) -> int:
        g = [[] for _ in range(n+1)]
        for [u, v, t] in times:
            g[u].append((v, t))
        
        total_time = [float('inf') for _ in range(n+1)]
        total_time[0] = 0
        total_time[k] = 0

        h = [(0, k)]
        while h:
            time, node = heappop(h)
            for neighbor, t in g[node]:
                if time + t < total_time[neighbor]:
                    total_time[neighbor] = time + t
                    heappush(h, (time + t, neighbor))
        
        res = 0
        for time in total_time:
            if time == float('inf'):
                return -1
            else:
                res = max(res, time)

        return res