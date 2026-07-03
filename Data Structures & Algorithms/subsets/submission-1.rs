impl Solution {
    pub fn subsets(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut all_subsets: Vec<Vec<i32>> = vec![];
        let mut temp: Vec<i32> = vec![];

        Self::f(&nums, 0, &mut temp, &mut all_subsets);
        
        all_subsets
    }

    pub fn f(nums: &[i32], i: usize, temp: &mut Vec<i32>, all_subsets: &mut Vec<Vec<i32>>) {
        if i == nums.len() {
            all_subsets.push(temp.clone());
            return;
        }

        // include the current index element
        temp.push(nums[i]);
        Self::f(nums, i+1, temp, all_subsets);

        // exclude the current index element
        temp.pop();
        Self::f(nums, i+1, temp, all_subsets);
    }
}
