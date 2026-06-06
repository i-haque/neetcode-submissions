use std::cmp::max;

impl Solution {
    pub fn largest_rectangle_area(heights: Vec<i32>) -> i32 {
        let n = heights.len();
        let mut stack: Vec<(i32, i32)> = Vec::new();

        let mut next_smallest_left = vec![-1; n];
        for i in 0..n {
            while !stack.is_empty() && stack.last().unwrap().1 >= heights[i] {
                stack.pop();
            }
            if !stack.is_empty() {
                next_smallest_left[i] = stack.last().unwrap().0;
            }
            stack.push((i as i32, heights[i]));
        }

        stack.clear();

        let mut next_smallest_right = vec![n as i32; n];
        for i in (0..n).rev() {
            while !stack.is_empty() && stack.last().unwrap().1 >= heights[i] {
                stack.pop();
            }
            if !stack.is_empty() {
                next_smallest_right[i] = stack.last().unwrap().0;
            }
            stack.push((i as i32, heights[i]));
        }

        let mut max_area = 0;
        for i in 0..n {
            let curr_area = heights[i] * (next_smallest_right[i] - next_smallest_left[i] - 1);
            max_area = max(max_area, curr_area);
        }

        max_area
    }
}
