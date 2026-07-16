impl Solution {
    pub fn subsets_with_dup(mut nums: Vec<i32>) -> Vec<Vec<i32>> {
        nums.sort_unstable();

        let mut subsets: Vec<Vec<i32>> = vec![];
        let mut temp: Vec<i32> = vec![];

        Self::f(&nums, 0, &mut temp, &mut subsets);
        subsets
    }

    pub fn f(nums: &[i32], mut i: usize, temp: &mut Vec<i32>, subsets: &mut Vec<Vec<i32>>) {
        if i == nums.len() {
            subsets.push(temp.clone());
            return;
        }

        // include
        temp.push(nums[i]);
        Self::f(nums, i+1, temp, subsets);
        temp.pop();

        // exclude
        while i+1 < nums.len() && nums[i] == nums[i+1] {
            i += 1;
        }
        Self::f(nums, i+1, temp, subsets);
    }
}
