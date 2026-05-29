impl Solution {
    pub fn length_of_longest_substring(s: String) -> i32 {
        let s: Vec<char> = s.chars().collect();

        let mut seen = HashSet::new();
        let mut res = 0;
        let mut j = 0;

        for i in 0..s.len() {
            while seen.contains(&s[i]) {
                seen.remove(&s[j]);
                j += 1;
            }

            seen.insert(s[i]);
            res = max(res, i-j+1)
        }

        res as i32
    }
}
