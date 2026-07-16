class Solution:
    def __init__(self):
        self.subsets = set()

    def subsetsWithDup(self, nums: List[int]) -> List[List[int]]:
        nums.sort()
        self.f(nums, 0, [])
        return list(self.subsets)
    
    def f(self, nums, i, temp):
        if i == len(nums):
            self.subsets.add(tuple(temp[:]))
            return
        
        # include
        temp.append(nums[i])
        self.f(nums, i+1, temp)

        # exclude
        temp.pop()
        self.f(nums, i+1, temp)