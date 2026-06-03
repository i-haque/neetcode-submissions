impl Solution {
    pub fn is_valid(s: String) -> bool {
        let mut stack = vec![];

        for ch in s.chars() {
            if ch == ')' {
                if !stack.is_empty() && *stack.last().unwrap() == '(' {
                    stack.pop();
                } else{
                    return false;
                }
            } else if ch == '}' {
                if !stack.is_empty() && *stack.last().unwrap() == '{' {
                    stack.pop();
                } else{
                    return false;
                }
            } else if ch == ']' {
                if !stack.is_empty() && *stack.last().unwrap() == '[' {
                    stack.pop();
                } else{
                    return false;
                }
            } else {
                stack.push(ch);
            }
        }

        stack.is_empty()
    }
}
