impl Solution {
    pub fn can_jump(nums: Vec<i32>) -> bool {
        let n = nums.len();

        let mut goal = n-1;
        for i in (0..n-1).rev() {
            if nums[i] as usize + i >= goal {
                goal = i;
            }
        }

        goal == 0
    }
}
