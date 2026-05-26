class Solution:
    def longestConsecutive(self, nums: List[int]) -> int:
        uniques = set(nums)
        max_length = 0
        for num in nums:
            if (num - 1) not in uniques:
                curr_length = 1
                while (num + 1) in uniques:
                    curr_length += 1
                    num += 1
                max_length = max(max_length, curr_length)
        return max_length