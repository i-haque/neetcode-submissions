impl Solution {
    pub fn combination_sum2(mut candidates: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
        candidates.sort_unstable();

        let mut all_combinations: Vec<Vec<i32>> = vec![];
        let mut temp: Vec<i32> = vec![];
        Self::f(&candidates, target, 0, 0, &mut temp, &mut all_combinations);

        all_combinations
    }

    pub fn f(candidates: &[i32], target: i32, mut i: usize, mut curr_sum: i32, temp: &mut Vec<i32>, all_combinations: &mut Vec<Vec<i32>>) {
        if curr_sum == target {
            all_combinations.push(temp.clone());
            return;
        }

        for index in (i..candidates.len()) {
            if index > i && candidates[index-1] == candidates[index] {
                continue;
            }

            // include
            curr_sum += candidates[index];
            temp.push(candidates[index]);

            if curr_sum <= target {
                Self::f(candidates, target, index+1, curr_sum, temp, all_combinations);
            }

            // exclude
            curr_sum -= candidates[index];
            temp.pop();
        }
    }
}
