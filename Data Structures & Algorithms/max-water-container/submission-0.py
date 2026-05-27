class Solution:
    def maxArea(self, heights: List[int]) -> int:
        max_area = 0
        start, end = 0, len(heights)-1
        while start < end:
            curr_area = min(heights[start], heights[end]) * (end - start)
            max_area = max(max_area, curr_area)

            if heights[start] < heights[end]:
                start += 1
            else:
                end -= 1
                
        return max_area