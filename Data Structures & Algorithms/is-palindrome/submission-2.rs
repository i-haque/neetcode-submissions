impl Solution {
    pub fn is_palindrome(s: String) -> bool {
        let s: Vec<char> = s.chars()
                        .filter(|c| c.is_ascii_alphanumeric())
                        .map(|c| c.to_ascii_lowercase())
                        .collect();
        let (mut start, mut end) = (0, s.len());
        while start < end {
            if s[start] != s[end-1] {
                return false;
            }
            start += 1;
            end -= 1;
        }
        true
    }
}
