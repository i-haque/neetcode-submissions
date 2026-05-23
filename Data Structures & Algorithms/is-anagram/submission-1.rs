impl Solution {
    pub fn is_anagram(s: String, t: String) -> bool {
        let mut freq: HashMap<char, u32> = HashMap::new();
        for ch in s.chars() {
            freq.entry(ch).and_modify(|val| *val += 1).or_insert(1);
        }
        for ch in t.chars() {
            if freq.contains_key(&ch) {
                freq.entry(ch).and_modify(|val| *val -= 1);
                if *freq.get(&ch).unwrap() == 0 {
                    freq.remove(&ch);
                }
            } else {
                return false;
            }
        }
        freq.len() == 0
    }
}
