impl Solution {
    pub fn permute(mut nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut res: Vec<Vec<i32>> = vec![];
        Self::f(&mut nums, 0, &mut res);
        res
    }

    pub fn f(nums: &mut Vec<i32>, index: usize, res: &mut Vec<Vec<i32>>) {
        if index == nums.len() {
            res.push(nums.clone());
            return;
        }

        for i in index..nums.len() {
            nums.swap(index, i);
            Self::f(nums, index + 1, res);
            nums.swap(index, i);
        }
    }
}
