impl Solution {
    pub fn find_duplicate(mut nums: Vec<i32>) -> i32 {
        let mut ans = -1;

        for i in 0..nums.len() {
            let index: usize = (nums[i]).abs() as usize;
            if nums[index - 1] < 0 {
                ans = index as i32;
                break;
            } else {
                nums[index - 1] *= -1;
            }
        }

        ans
    }
}
