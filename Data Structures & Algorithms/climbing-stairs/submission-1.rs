impl Solution {
    pub fn climb_stairs(n: i32) -> i32 {
        if n < 3 {
            return n;
        }

        let (mut a, mut b) = (1, 2);
        for _ in 0..n-2 {
            let temp = b;
            b = a + b;
            a = temp;
        }

        b
    }
}
