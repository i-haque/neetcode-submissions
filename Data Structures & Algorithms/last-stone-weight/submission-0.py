from heapq import heapify, heappush, heappop

class Solution:
    def lastStoneWeight(self, stones: List[int]) -> int:
        stones = [-stone for stone in stones]
        heapify(stones)

        while len(stones) > 1:
            x = -heappop(stones)
            y = -heappop(stones)

            if x > y:
                heappush(stones, -(x - y))
            elif y > x:
                heappush(stones, -(y - x))
        
        return 0 if len(stones) == 0 else -stones[0]