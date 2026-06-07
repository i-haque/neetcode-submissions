class Solution:
    def search(self, nums: List[int], target: int) -> int:
        low, high = 0, len(nums)

        while low < high:
            mid = low + (high - low) // 2
            if nums[mid] == target:
                return mid
            elif target < nums[mid]:
                high = mid
            elif nums[mid] < target:
                low = mid + 1
        
        return -1