impl Solution {
    pub fn three_sum(mut nums: Vec<i32>) -> Vec<Vec<i32>> {
        nums.sort_unstable();

        let n = nums.len();
        let mut triplets = Vec::new();

        for i in 0..n-2 {
            if nums[i] > 0 {
                break;
            }

            if i > 0 && nums[i-1] == nums[i] {
                continue;
            }

            let (mut j, mut k) = (i+1, n-1);
            while j < k {
                let total = nums[i] + nums[j] + nums[k];

                if total > 0 {
                    k -= 1;
                } else if total < 0 {
                    j += 1;
                } else {
                    triplets.push(vec![nums[i], nums[j], nums[k]]);

                    while j < k && nums[j] == nums[j+1] {
                        j += 1;
                    }
                    while j < k && nums[k-1] == nums[k] {
                        k -= 1;
                    }

                    j += 1;
                    k -= 1;
                }
            }
        }
        
        triplets
    }
}
