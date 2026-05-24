from heapq import heappush, heappop

class Solution:
    def topKFrequent(self, nums: List[int], k: int) -> List[int]:
        freq = defaultdict(int)
        for num in nums:
            freq[num] += 1
        h = []
        for (key, val) in freq.items():
            heappush(h, (val, key))
            if len(h) > k:
                heappop(h)
        res = []
        for (_, key) in h:
            res.append(key)
        return res