"""
Definition of Interval:
class Interval(object):
    def __init__(self, start, end):
        self.start = start
        self.end = end
"""

class Solution:
    def canAttendMeetings(self, intervals: List[Interval]) -> bool:
        intervals = [(interval.start, interval.end) for interval in intervals]
        if not intervals:
            return True

        intervals.sort()
        (start, end) = intervals[0]

        for i in range(1, len(intervals)):
            [curr_start, curr_end] = intervals[i]
            if end > curr_start:
                return False
            else:
                start, end = curr_start, curr_end
        
        return True