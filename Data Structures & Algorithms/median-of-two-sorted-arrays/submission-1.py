class Solution:
    def findMedianSortedArrays(self, nums1: List[int], nums2: List[int]) -> float:
        n1, n2 = len(nums1), len(nums2)
        if n2 < n1:
            nums1, nums2 = nums2, nums1
            n1, n2 = n2, n1
        
        total = n1 + n2
        half = total // 2

        low, high = 0, n1 - 1
        while True:
            mid1 = low + (high - low) // 2
            mid2 = half - mid1 - 2

            nums1_left = nums1[mid1] if mid1 >= 0 else float('-inf')
            nums2_left = nums2[mid2] if mid2 >= 0 else float('-inf')
            nums1_right = nums1[mid1 + 1] if (mid1 + 1) < n1 else float('inf')
            nums2_right = nums2[mid2 + 1] if (mid2 + 1) < n2 else float('inf')

            if nums1_left <= nums2_right and nums2_left <= nums1_right:
                # odd/even
                if total % 2 == 1:
                    return min(nums1_right, nums2_right)
                else:
                    return (max(nums1_left, nums2_left) + min(nums1_right, nums2_right)) / 2
            elif nums1_left > nums2_right:
                high = mid1 - 1
            else:
                low = mid1 + 1
        
        return 0.0