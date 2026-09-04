impl Solution {
    pub fn can_partition(nums: Vec<i32>) -> bool {
        let total_sum: i32 = nums.iter().sum();
        if total_sum % 2 == 1 {
            return false;
        }

        let n = nums.len();
        let half_sum = (total_sum / 2) as usize;
        let mut dp: Vec<Vec<bool>> = vec![vec![false; half_sum+1]; n+1];
        for i in 0..n+1 {
            dp[i][0] = true;
        }

        for i in 1..n+1 {
            for j in 1..half_sum+1 {
                if nums[i-1] <= j as i32 {
                    dp[i][j] = dp[i-1][j - nums[i-1] as usize] || dp[i-1][j];
                } else {
                    dp[i][j] = dp[i-1][j];
                }
            }
        }
        
        dp[n][half_sum]
    }
}
