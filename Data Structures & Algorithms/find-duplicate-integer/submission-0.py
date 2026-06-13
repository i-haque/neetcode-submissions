class Solution:
    def findDuplicate(self, nums: List[int]) -> int:
        ans = -1

        for i in range(len(nums)):
            index = abs(nums[i])
            if nums[index - 1] < 0:
                ans = index
                break
            else:
                nums[index - 1] *= -1
        
        return ans