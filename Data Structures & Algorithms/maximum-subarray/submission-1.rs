use std::cmp::max;

impl Solution {
    pub fn max_sub_array(nums: Vec<i32>) -> i32 {
        let mut max_sum = i32::MIN;
        let mut curr_sum = 0;

        for num in nums {
            curr_sum = max(curr_sum + num, num);
            max_sum = max(max_sum, curr_sum);
        }

        max_sum
    }
}
