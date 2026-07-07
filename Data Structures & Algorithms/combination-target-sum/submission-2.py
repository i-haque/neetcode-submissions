class Solution:
    def __init__(self):
        self.combinations = []

    def combinationSum(self, nums: List[int], target: int) -> List[List[int]]:
        self.f(nums, target, 0, 0, [])
        return self.combinations
        
    def f(self, nums, target, i, curr_sum, temp):
        # base cases
        if curr_sum == target:
            self.combinations.append(temp[:])
            return
        
        for index in range(i, len(nums)):
            curr_sum += nums[index]
            temp.append(nums[index])

            if curr_sum <= target:
                self.f(nums, target, index, curr_sum, temp)
            
            curr_sum -= nums[index]
            temp.pop()
            
