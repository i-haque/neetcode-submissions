class Solution:
    def __init__(self):
        self.all_subsets = []

    def subsets(self, nums: List[int]) -> List[List[int]]:
        self.f(nums, 0, [])
        return self.all_subsets
    
    def f(self, nums, i, temp):
        if i == len(nums):
            self.all_subsets.append(temp[:])
            return
        
        # include the element
        temp.append(nums[i])
        self.f(nums, i+1, temp)

        # don't include the element
        temp.pop()
        self.f(nums, i+1, temp)
