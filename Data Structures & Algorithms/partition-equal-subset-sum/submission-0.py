class Solution:
    def __init__(self):
        self.dp = dict()

    def canPartition(self, nums: List[int]) -> bool:
        total_sum = sum(nums)
        if total_sum % 2 == 1:
            return False

        half_sum = total_sum // 2
        return self.f(nums, 0, half_sum)
    
    def f(self, nums, i, half_sum):
        if half_sum == 0:
            return True
        if i == len(nums):
            return False

        # memoization
        if (i, half_sum) in self.dp:
            return self.dp[(i, half_sum)]

        res = False
        # take
        if nums[i] <= half_sum:
            res = self.f(nums, i+1, half_sum - nums[i]) or self.f(nums, i+1, half_sum)
        # don't take
        else:
            res = self.f(nums, i+1, half_sum)
        
        self.dp[(i, half_sum)] = res
        return res
