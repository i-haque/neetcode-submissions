from heapq import heappush, heappop
import math

class Solution:
    def kClosest(self, points: List[List[int]], k: int) -> List[List[int]]:
        max_heap = []

        for [x, y] in points:
            dist = math.sqrt((x * x) + (y * y))
            heappush(max_heap, (-dist, [x, y]))

            if len(max_heap) > k:
                heappop(max_heap)
        
        return [points for [_, points] in max_heap]