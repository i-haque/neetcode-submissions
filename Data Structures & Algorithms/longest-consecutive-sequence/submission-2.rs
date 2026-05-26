impl Solution {
    pub fn longest_consecutive(nums: Vec<i32>) -> i32 {
        let uniques: HashSet<&i32> = nums.iter().collect();
        let mut max_length: i32 = 0;
        for num in &nums {
            let mut n: i32 = *num;
            if !uniques.contains(&(n - 1)) {
                let mut curr_length: i32 = 1;
                while uniques.contains(&(n + 1)) {
                    curr_length += 1;
                    n += 1;
                }
                max_length = std::cmp::max(max_length, curr_length);
            } 
        }
        max_length
    }
}
