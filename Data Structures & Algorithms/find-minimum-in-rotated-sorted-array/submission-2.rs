impl Solution {
    pub fn find_min(nums: Vec<i32>) -> i32 {
        let (mut low, mut high) = (0, (nums.len() - 1) as i32);

        while low < high {
            let mid = low + (high - low) / 2;

            if nums[mid as usize] > nums[high as usize] {
                low = mid + 1;
            } else {
                high = mid;
            }
        }

        nums[high as usize]
    }
}
