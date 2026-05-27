class Solution:
    def threeSum(self, nums: List[int]) -> List[List[int]]:
        n = len(nums)
        nums.sort()
        triplets = set()
        for i in range(n):
            j, k = i + 1, n - 1
            while j < k:
                if nums[i] + nums[j] + nums[k] > 0:
                    k -= 1
                elif nums[i] + nums[j] + nums[k] < 0:
                    j += 1
                else:
                    triplets.add((nums[i], nums[j], nums[k]))
                    j += 1
                    k -= 1
        return [list(triplet) for triplet in triplets]