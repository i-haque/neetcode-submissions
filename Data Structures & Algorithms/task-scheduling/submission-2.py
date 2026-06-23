from heapq import heappush, heappop

class Solution:
    def leastInterval(self, tasks: List[str], n: int) -> int:
        freq = dict()
        for task in tasks:
            freq[task] = freq.get(task, 0) + 1
        
        min_heap = []
        for val in freq.values():
            heappush(min_heap, (1, val))
        
        total_time = 0
        while min_heap:
            (time, count) = heappop(min_heap)
            if total_time >= time:
                total_time += 1
            else:
                total_time = time

            if count - 1 > 0:
                heappush(min_heap, (time + n + 1, count - 1))
            
        return total_time