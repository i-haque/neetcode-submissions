impl Solution {
    pub fn letter_combinations(digits: String) -> Vec<String> {
        if digits.is_empty() {
            return vec![];
        }

        let map: HashMap<char, Vec<char>> = HashMap::from([
            ('2', vec!['a', 'b', 'c']),
            ('3', vec!['d', 'e', 'f']),
            ('4', vec!['g', 'h', 'i']),
            ('5', vec!['j', 'k', 'l']),
            ('6', vec!['m', 'n', 'o']),
            ('7', vec!['p', 'q', 'r', 's']),
            ('8', vec!['t', 'u', 'v']),
            ('9', vec!['w', 'x', 'y', 'z'])
        ]);
        let digits: Vec<char> = digits.chars().collect();
        let mut temp = String::new();
        let mut combinations: Vec<String> = vec![];

        Self::dfs(&map, &digits, 0, &mut temp, &mut combinations);
        combinations
    }

    pub fn dfs(map: &HashMap<char, Vec<char>>, digits: &[char], i: usize, temp: &mut String, combinations: &mut Vec<String>) {
        if i == digits.len() {
            combinations.push(temp.clone());
            return;
        }

        if let Some(characters) = map.get(&digits[i]) {
            for ch in characters {
                temp.push(*ch);
                Self::dfs(map, digits, i+1, temp, combinations);
                temp.pop();
            }
        }
    }
}
