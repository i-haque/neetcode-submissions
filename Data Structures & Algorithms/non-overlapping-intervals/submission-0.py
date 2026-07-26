class Solution:
    def eraseOverlapIntervals(self, intervals: List[List[int]]) -> int:
        intervals.sort()
        removals = 0
        [prev_start, prev_end] = intervals[0]

        for i in range(1, len(intervals)):
            [curr_start, curr_end] = intervals[i]
            if prev_end > curr_start:
                prev_end = min(prev_end, curr_end)
                removals += 1
            else:
                prev_start, prev_end = curr_start, curr_end
        
        return removals