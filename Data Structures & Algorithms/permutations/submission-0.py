class Solution:
    def __init__(self):
        self.permutations = []

    def permute(self, nums: List[int]) -> List[List[int]]:
        self.f(nums, 0)
        return self.permutations
        
    def f(self, nums, index):
        if index == len(nums):
            self.permutations.append(nums[:])
            return
        
        for i in range(index, len(nums)):
            nums[index], nums[i] = nums[i], nums[index]
            self.f(nums, index + 1)
            nums[index], nums[i] = nums[i], nums[index]