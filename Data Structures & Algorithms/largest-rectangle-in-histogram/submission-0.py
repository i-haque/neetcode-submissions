class Solution:
    def largestRectangleArea(self, heights: List[int]) -> int:
        n = len(heights)
        stack = []

        next_smallest_left = [-1] * n
        for i in range(n):
            while stack and stack[-1][1] >= heights[i]:
                stack.pop()
            if stack:
                next_smallest_left[i] = stack[-1][0]
            stack.append((i, heights[i]))
        
        stack.clear()

        next_smallest_right = [n] * n
        for i in range(n-1, -1, -1):
            while stack and stack[-1][1] >= heights[i]:
                stack.pop()
            if stack:
                next_smallest_right[i] = stack[-1][0]
            stack.append((i, heights[i]))
        
        max_area = 0
        for i in range(n):
            curr_area = heights[i] * (next_smallest_right[i] - next_smallest_left[i] - 1)
            max_area = max(max_area, curr_area)
        
        return max_area
