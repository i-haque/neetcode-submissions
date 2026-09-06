impl Solution {
    pub fn rob(mut nums: Vec<i32>) -> i32 {
        let n = nums.len() as i32;

        for i in 1..n {
            let m1 = nums[(i-1) as usize];
            let mut m2 = 0;
            if i - 2 >= 0 {
                m2 = nums[(i-2) as usize];
            }
            nums[i as usize] = max(nums[i as usize] + m2, m1);
        }

        *nums.last().unwrap()
    }
}
