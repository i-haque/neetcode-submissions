impl Solution {
    pub fn product_except_self(nums: Vec<i32>) -> Vec<i32> {
        let n: usize = nums.len();

        let mut curr: i32 = 1;
        let mut prefix: Vec<i32> = vec![1; n];
        for i in 1..n {
            curr *= nums[i-1];
            prefix[i] = curr;
        }

        let mut curr: i32 = 1;
        let mut suffix: Vec<i32> = vec![1; n];
        for i in (0..n-1).rev() {
            curr *= nums[i+1];
            suffix[i] = curr;
        }

        prefix.into_iter().zip(suffix.into_iter()).map(|(a,b)| a*b).collect::<Vec<i32>>()
    }
}
