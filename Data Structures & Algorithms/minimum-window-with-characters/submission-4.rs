impl Solution {
    pub fn min_window(s: String, t: String) -> String {
        let s: Vec<char> = s.chars().collect();
        let (n1, n2) = (s.len(), t.len());
        if n2 == 0 || n2 > n1 {
            return String::from("");
        }

        let mut freq_t = HashMap::new();
        for ch in t.chars() {
            *freq_t.entry(ch).or_insert(0) += 1;
        }

        let mut res: (usize, usize) = (0, 0);
        let mut length = usize::MAX;

        let (mut need, mut have) = (freq_t.len() as i32, 0);
        let mut freq_s = HashMap::new();
        let mut j = 0;
        for i in 0..n1 {
            if freq_t.contains_key(&s[i]) {
                *freq_s.entry(s[i]).or_insert(0) += 1;
                if freq_t.get(&s[i]) == freq_s.get(&s[i]) {
                    have += 1;
                }
            }

            while have == need {
                if (i-j+1) < length {
                    res = (j, i);
                    length = i-j+1;
                }

                if freq_t.contains_key(&s[j]) {
                    freq_s.entry(s[j]).and_modify(|val| *val -= 1);
                    if freq_s.get(&s[j]) < freq_t.get(&s[j]) {
                        have -= 1;
                    }
                }
                j += 1;
            }
        }

        if length == usize::MAX {
            return String::from("");
        }

        s[res.0..res.1+1].iter().collect::<String>()
    }
}
