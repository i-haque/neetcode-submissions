class Solution:
    def __init__(self):
        self.subsets = []

    def subsetsWithDup(self, nums: List[int]) -> List[List[int]]:
        nums.sort()
        self.f(nums, 0, [])
        return self.subsets
    
    def f(self, nums, i, temp):
        if i == len(nums):
            self.subsets.append(temp[:])
            return
        
        # include
        temp.append(nums[i])
        self.f(nums, i+1, temp)
        temp.pop()

        # exclude
        while i+1 < len(nums) and nums[i] == nums[i+1]:
            i += 1
        
        self.f(nums, i+1, temp)