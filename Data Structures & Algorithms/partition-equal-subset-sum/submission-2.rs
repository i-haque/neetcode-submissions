use std::collections::HashMap;

impl Solution {
    pub fn can_partition(nums: Vec<i32>) -> bool {
        let total_sum: i32 = nums.iter().sum();
        if total_sum % 2 == 1 {
            return false;
        }

        let half_sum = total_sum / 2;
        let mut dp: HashMap<(usize, i32), bool> = HashMap::new();
        Self::f(&nums, 0, half_sum, &mut dp)
    }

    pub fn f(nums: &[i32], mut i: usize, mut half_sum: i32, dp: &mut HashMap<(usize, i32), bool>) -> bool {
        if half_sum == 0 {
            return true;
        }
        if i == nums.len() {
            return false;
        }

        if let Some(val) = dp.get(&(i, half_sum)) {
            return *val;
        }

        let mut res = false;
        if nums[i] <= half_sum {
            res = Self::f(nums, i+1, half_sum - nums[i], dp) || Self::f(nums, i+1, half_sum, dp);
        } else {
            res = Self::f(nums, i+1, half_sum, dp);
        }

        dp.insert((i, half_sum), res);
        res
    }
}
