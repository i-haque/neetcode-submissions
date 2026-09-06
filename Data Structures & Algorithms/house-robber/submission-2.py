class Solution:
    def rob(self, nums: List[int]) -> int:
        n = len(nums)
        if n < 2:
            return nums[0]

        for i in range(1, n):
            m1 = nums[i-1]
            m2 = 0 if i - 2 < 0 else nums[i-2]
            nums[i] = max(nums[i] + m2, m1)
        
        return nums[-1]