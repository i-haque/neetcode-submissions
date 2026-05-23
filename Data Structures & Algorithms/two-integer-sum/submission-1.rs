impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut seen: HashMap<i32, i32> = HashMap::new();
        for (i, num) in nums.iter().enumerate() {
            if seen.contains_key(&(target - *num)) {
                return vec![*seen.get(&(target - *num)).unwrap(), i as i32];
            } else {
                seen.insert(*num, i as i32);
            }
        }
        vec![]
    }
}
