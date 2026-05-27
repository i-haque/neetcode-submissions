impl Solution {
    pub fn trap(height: Vec<i32>) -> i32 {
        let n = height.len();
        if n == 0 {
            return 0;
        }

        let (mut max_left, mut max_right) = (height[0], height[n-1]);
        let mut trapped_water = 0;
        let (mut left, mut right) = (0, n-1);

        while left < right {
            if max_left <= max_right {
                left += 1;
                max_left = max(max_left, height[left]);
                trapped_water += (max_left - height[left]);
            } else {
                right -= 1;
                max_right = max(max_right, height[right]);
                trapped_water += (max_right - height[right]);
            }
        }

        trapped_water
    }
}
