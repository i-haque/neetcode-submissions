from heapq import heappush, heappop

class Solution:
    def minInterval(self, intervals: List[List[int]], queries: List[int]) -> List[int]:
        queries = [(q, i) for i, q in enumerate(queries)]
        queries.sort(key = lambda x: x[0])
        intervals.sort(key = lambda x: x[0])

        res = [-1] * len(queries)

        h = []
        j = 0
        for (q, i) in queries:
            while j < len(intervals) and intervals[j][0] <= q:
                [start, end] = intervals[j]
                heappush(h, (end - start + 1, end))
                j += 1
            
            while h and h[0][1] < q:
                heappop(h)
            
            if h:
                res[i] = h[0][0]
        
        return res
