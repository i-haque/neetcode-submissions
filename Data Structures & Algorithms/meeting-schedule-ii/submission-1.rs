/**
 * Definition of Interval:
 * #[derive(Debug, Clone)]
 * pub struct Interval {
 *     pub start: i32,
 *     pub end: i32,
 * }
 *
 * impl Interval {
 *     pub fn new(start: i32, end: i32) -> Self {
 *         Interval { start, end }
 *     }
 * }
 */

 use std::cmp::max;

impl Solution {
    pub fn min_meeting_rooms(intervals: Vec<Interval>) -> i32 {
        let n = intervals.len();
        let mut start: Vec<i32> = vec![0; n];
        let mut end: Vec<i32> = vec![0; n];

        for (i, interval) in intervals.into_iter().enumerate() {
            start[i] = interval.start;
            end[i] = interval.end;
        }
        start.sort_unstable();
        end.sort_unstable();

        let mut max_rooms = 0;
        let mut rooms = 0;

        let mut s = 0;
        let mut e = 0;
        while s < n {
            if start[s] < end[e] {
                rooms += 1;
                s += 1;
            } else {
                rooms -= 1;
                e += 1;
            }
            max_rooms = max(max_rooms, rooms);
        }

        max_rooms
    }
}
