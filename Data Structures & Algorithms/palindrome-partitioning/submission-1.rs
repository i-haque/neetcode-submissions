impl Solution {
    pub fn partition(s: String) -> Vec<Vec<String>> {
        let mut partitions: Vec<String> = vec![];
        let mut res: Vec<Vec<String>> = vec![];
        Self::dfs(&s, 0, &mut partitions, &mut res);
        res
    }

    pub fn dfs(s: &str, i: usize, partitions: &mut Vec<String>, res: &mut Vec<Vec<String>>) {
        if i == s.len() {
            res.push(partitions.clone());
            return;
        }

        for j in i..s.len() {
            if Self::is_pallindrome(&s[i..j+1]) {
                partitions.push(s[i..j+1].to_string());
                Self::dfs(s, j+1, partitions, res);
                partitions.pop();
            }
        }
    }

    pub fn is_pallindrome(substr: &str) -> bool {
        substr.chars().eq(substr.chars().rev())
    }
}
