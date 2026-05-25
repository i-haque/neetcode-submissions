class Solution:
    def productExceptSelf(self, nums: List[int]) -> List[int]:
        n = len(nums)
        prefix = [1] * n
        curr = 1
        for i in range(1, n):
            curr *= nums[i-1]
            prefix[i] = curr

        suffix = [1] * n
        curr = 1
        for i in range(n-2, -1, -1):
            curr *= nums[i+1]
            suffix[i] = curr
        
        return [a*b for (a, b) in zip(prefix, suffix)]