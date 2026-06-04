impl Solution {
    pub fn daily_temperatures(temperatures: Vec<i32>) -> Vec<i32> {
        let n = temperatures.len();
        let mut res = vec![0; n];
        let mut stack: Vec<(usize, i32)> = vec![];

        for i in (0..n).rev() {
            while !stack.is_empty() && stack.last().unwrap().1 <= temperatures[i] {
                stack.pop();
            }

            if !stack.is_empty() {
                res[i] = (stack.last().unwrap().0 - i) as i32;
            }

            stack.push((i, temperatures[i]));
        }

        res
    }
}
