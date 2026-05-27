impl Solution {
    pub fn two_sum(numbers: Vec<i32>, target: i32) -> Vec<i32> {
        let (mut start, mut end) = (0, numbers.len() - 1);
        while start < end {
            if numbers[start] + numbers[end] > target {
                end -= 1;
            } else if numbers[start] + numbers[end] < target {
                start += 1;
            } else {
                break;
            }
        }
        vec![(start + 1) as i32, (end + 1) as i32]
    }
}
