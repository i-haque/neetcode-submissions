impl Solution {
    pub fn eval_rpn(tokens: Vec<String>) -> i32 {
        let mut stack = vec![];

        for token in tokens {
            if token == "+" {
                let num2 = stack.pop().unwrap();
                let num1 = stack.pop().unwrap();
                stack.push(num1 + num2);
            } else if token == "-" {
                let num2 = stack.pop().unwrap();
                let num1 = stack.pop().unwrap();
                stack.push(num1 - num2);
            } else if token == "*" {
                let num2 = stack.pop().unwrap();
                let num1 = stack.pop().unwrap();
                stack.push(num1 * num2);
            } else if token == "/" {
                let num2 = stack.pop().unwrap();
                let num1 = stack.pop().unwrap();
                stack.push(num1 / num2);
            } else {
                stack.push(token.parse::<i32>().unwrap())
            }
        }

        stack.pop().unwrap()
    }
}
