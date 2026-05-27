class Solution:
    def threeSum(self, nums: List[int]) -> List[List[int]]:
        n = len(nums)
        nums.sort()
        triplets = []
        for i in range(n-2):
            if nums[i] > 0:
                break
            if i > 0 and nums[i-1] == nums[i]:
                continue
            j, k = i + 1, n - 1
            while j < k:
                total = nums[i] + nums[j] + nums[k]
                if total > 0:
                    k -= 1
                elif total < 0:
                    j += 1
                else:
                    triplets.append([nums[i], nums[j], nums[k]])
                    while j < k and nums[j] == nums[j+1]:
                        j += 1
                    while j < k and nums[k-1] == nums[k]:
                        k -= 1
                    j += 1
                    k -= 1
        return triplets