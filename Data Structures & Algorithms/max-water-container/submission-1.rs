use std::cmp::{min, max};

impl Solution {
    pub fn max_area(heights: Vec<i32>) -> i32 {
        let mut max_area = 0;
        let (mut start, mut end) = (0, heights.len() - 1);
        while start < end {
            let curr_area = min(heights[start], heights[end]) * (end - start) as i32;
            max_area = max(max_area, curr_area);

            if heights[start] < heights[end] {
                start += 1;
            } else {
                end -= 1;
            }
        }

        max_area
    }
}
