impl Solution {
    pub fn combination_sum(nums: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
        let mut all_combinations: Vec<Vec<i32>> = vec![];
        let mut temp: Vec<i32> = vec![];

        Self::f(&nums, target, 0, 0, &mut temp, &mut all_combinations);

        all_combinations
    }

    pub fn f(nums: &[i32], target: i32, i: usize, mut curr_sum: i32, temp: &mut Vec<i32>, all_combinations: &mut Vec<Vec<i32>>) {
        // base cases
        if curr_sum == target {
            all_combinations.push(temp.clone());
            return;
        }

        if i == nums.len() {
            return;
        }

        // include
        temp.push(nums[i]);
        curr_sum += nums[i];

        if curr_sum <= target {
            Self::f(nums, target, i, curr_sum, temp, all_combinations);
        }

        // exclude
        temp.pop();
        curr_sum -= nums[i];

        Self::f(nums, target, i+1, curr_sum, temp, all_combinations)
    }
}
