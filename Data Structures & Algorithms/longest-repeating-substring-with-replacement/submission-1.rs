impl Solution {
    pub fn character_replacement(s: String, k: i32) -> i32 {
        let s: Vec<char> = s.chars().collect();
        let n = s.len();
        let mut res = 0;

        let mut max_freq = 0;
        let mut freq = HashMap::new();

        let mut j = 0;
        for i in 0..n {
            *freq.entry(s[i]).or_insert(0) += 1;
            max_freq = max(max_freq, *freq.get(&s[i]).unwrap());

            while (i-j+1) as i32 - max_freq > k {
                freq.entry(s[j]).and_modify(|val| *val -= 1);
                j += 1;
            }

            res = max(res, i-j+1);
        }

        res as i32
    }
}
