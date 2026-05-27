class Solution:
    def trap(self, height: List[int]) -> int:
        n = len(height)
        if n == 0:
            return 0

        max_left, max_right = height[0], height[-1]
        trapped_water = 0

        left, right = 0, n-1
        while left < right:
            if max_left <= max_right:
                left += 1
                max_left = max(max_left, height[left])
                trapped_water += (max_left - height[left])
            else:
                right -= 1
                max_right = max(max_right, height[right])
                trapped_water += (max_right - height[right])
        
        return trapped_water