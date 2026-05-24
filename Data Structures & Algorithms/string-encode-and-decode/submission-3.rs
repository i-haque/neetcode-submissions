impl Solution {
    pub fn encode(strs: Vec<String>) -> String {
        let mut encoded: String = String::new();
        for s in &strs {
            encoded.push_str(&s.len().to_string());
            encoded.push('#');
            encoded.push_str(s);
        }
        encoded
    }

    pub fn decode(s: String) -> Vec<String> {
        let mut res: Vec<String> = vec![];
        if s.len() == 0 {
            return res;
        }
        let s: Vec<char> = s.chars().collect();
        let mut i: usize = 0;
        while i < s.len() {
            let mut length_str: String = String::new();
            while s[i] != '#' {
                length_str.push(s[i]);
                i += 1;
            }

            i += 1;

            let length: usize = length_str.parse().unwrap();

            let mut word: String = s[i..i+length].iter().collect();
            res.push(word);
            i += length;
        }
        res
    }
}
