impl Solution {
    pub fn group_anagrams(strs: Vec<String>) -> Vec<Vec<String>> {
        let mut groups: HashMap<String, Vec<String>> = HashMap::new();
        for s in strs {
            let key: String = Self::sort_string(&s);
            groups.entry(key).and_modify(|group| group.push(s.clone())).or_insert(vec![s]);
        }
        groups.into_iter().map(|(_, group)| group).collect::<Vec<Vec<String>>>()
    }

    pub fn sort_string(s: &str) -> String {
        let mut chars: Vec<char> = s.chars().collect();
        chars.sort_unstable();
        chars.into_iter().collect::<String>()
    }
}
