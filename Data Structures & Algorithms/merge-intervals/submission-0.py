class Solution:
    def merge(self, intervals: List[List[int]]) -> List[List[int]]:
        intervals.sort()
        merged = []

        start, end = intervals[0]
        for i in range(1, len(intervals)):
            curr_start, curr_end = intervals[i]

            if end >= curr_start:
                start = min(start, curr_start)
                end = max(end, curr_end)
            else:
                merged.append([start, end])
                start, end = curr_start, curr_end
        
        merged.append([start, end])
        return merged
