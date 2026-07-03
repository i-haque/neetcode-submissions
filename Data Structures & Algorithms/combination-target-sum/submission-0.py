class Solution:
    def __init__(self):
        self.combinations = []

    def combinationSum(self, nums: List[int], target: int) -> List[List[int]]:
        self.f(nums, target, 0, 0, [])
        return self.combinations
        
    def f(self, nums, target, curr_sum, i, temp):
        # base cases
        if curr_sum == target:
            self.combinations.append(temp[:])
            return

        if i == len(nums):
            return
        
        # include
        temp.append(nums[i])
        curr_sum += nums[i]

        if curr_sum <= target:
            self.f(nums, target, curr_sum, i, temp)

        # exclude
        temp.pop()
        curr_sum -= nums[i]
        self.f(nums, target, curr_sum, i+1, temp)