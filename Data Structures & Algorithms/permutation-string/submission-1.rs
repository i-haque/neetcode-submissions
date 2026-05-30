use std::collections::hash_map::Entry;

impl Solution {
    pub fn check_inclusion(s1: String, s2: String) -> bool {
        let s1: Vec<char> = s1.chars().collect();
        let s2: Vec<char> = s2.chars().collect();
        let (n1, n2) = (s1.len(), s2.len());

        let mut d1 = HashMap::new();
        for i in 0..n1 {
            *d1.entry(s1[i]).or_insert(0) += 1;
        }

        let mut d2 = HashMap::new();
        let mut j = 0;
        for i in 0..n2 {
            *d2.entry(s2[i]).or_insert(0) += 1;

            if (i-j+1) == n1 {
                if d1 == d2 {
                    return true;
                }

                if let Entry::Occupied(mut entry) = d2.entry(s2[j]) {
                    *entry.get_mut() -= 1;
                    if *entry.get() == 0 {
                        entry.remove();
                    }
                }
                j += 1;
            }
        }

        false
    }
}
