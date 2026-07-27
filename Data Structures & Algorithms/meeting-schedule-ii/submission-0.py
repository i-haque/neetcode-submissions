"""
Definition of Interval:
class Interval(object):
    def __init__(self, start, end):
        self.start = start
        self.end = end
"""

class Solution:
    def minMeetingRooms(self, intervals: List[Interval]) -> int:
        n = len(intervals)
        if n == 0:
            return 0

        start = [0] * n
        end = [0] * n
        for i, interval in enumerate(intervals):
            start[i] = interval.start
            end[i] = interval.end
        start.sort()
        end.sort()

        max_rooms = rooms = 0
        s = e = 0
        while s < n:
            if start[s] < end[e]:
                rooms += 1
                s += 1
            else:
                rooms -= 1
                e += 1
            max_rooms = max(max_rooms, rooms)
        
        return max_rooms