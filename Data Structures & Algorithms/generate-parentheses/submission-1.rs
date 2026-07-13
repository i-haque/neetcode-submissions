impl Solution {
    pub fn generate_parenthesis(n: i32) -> Vec<String> {
        let mut res: Vec<String> = vec![];
        let mut temp: Vec<char> = vec![];

        Self::f(n, 0, 0, &mut temp, &mut res);
        res
    }

    pub fn f(n: i32, op: i32, cl: i32, temp: &mut Vec<char>, res: &mut Vec<String>) {
        if op == n && cl == n {
            res.push(temp.iter().collect());
            return;
        }

        if op < n {
            temp.push('(');
            Self::f(n, op + 1, cl, temp, res);
            temp.pop();
        }

        if op > cl {
            temp.push(')');
            Self::f(n, op, cl + 1, temp, res);
            temp.pop();
        }
    }
}
