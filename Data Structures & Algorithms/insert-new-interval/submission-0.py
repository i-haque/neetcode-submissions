class Solution:
    def insert(self, intervals: List[List[int]], newInterval: List[int]) -> List[List[int]]:
        intervals.append(newInterval)
        intervals.sort()
        n = len(intervals)
        
        merged = []
        start, end = intervals[0]

        for i in range(1, n):
            [curr_start, curr_end] = intervals[i]

            if end >= curr_start:
                start = min(start, curr_start)
                end = max(end, curr_end)
            else:
                merged.append([start, end])
                start, end = curr_start, curr_end
        
        merged.append([start, end])
        return merged