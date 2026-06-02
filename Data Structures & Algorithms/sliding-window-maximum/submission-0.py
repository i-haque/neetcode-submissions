class Solution:
    def maxSlidingWindow(self, nums: List[int], k: int) -> List[int]:
        res = []
        q = deque()

        for (i, num) in enumerate(nums):
            while q and q[-1][0] < num:
                q.pop()
            q.append((num, i))

            if i >= k-1:
                res.append(q[0][0])

            if i - q[0][1] == k-1:
                q.popleft()
        
        return res