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

impl Solution {
    pub fn can_attend_meetings(intervals: Vec<Interval>) -> bool {
        let mut intervals: Vec<(i32, i32)> = intervals
            .into_iter()
            .map(|interval| (interval.start, interval.end))
            .collect();
        if intervals.is_empty() {
            return true;
        }

        intervals.sort_unstable();
        let (mut start, mut end) = intervals[0];

        for i in 1..intervals.len() {
            let (curr_start, curr_end) = intervals[i];
            if end > curr_start {
                return false;
            } else {
                (start, end) = (curr_start, curr_end);
            }
        }

        true
    }
}
