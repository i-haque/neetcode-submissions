class Solution:
    def searchMatrix(self, matrix: List[List[int]], target: int) -> bool:
        m, n = len(matrix), len(matrix[0])
        low, high = 0, m*n

        while low < high:
            mid = low + (high - low) // 2
            if matrix[mid // n][mid % n] == target:
                return True
            elif target < matrix[mid // n][mid % n]:
                high = mid
            elif matrix[mid // n][mid % n] < target:
                low = mid + 1
        
        return False