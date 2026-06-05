impl Solution {
    pub fn car_fleet(target: i32, position: Vec<i32>, speed: Vec<i32>) -> i32 {
        let mut cars: Vec<(i32, i32)> = position.into_iter()
                        .zip(speed.into_iter())
                        .map(|(pos, sp)| (pos, sp))
                        .collect();
        cars.sort_unstable_by_key(|item| item.0);

        let mut stack = vec![];
        for (pos, sp) in cars {
            let time = (target - pos) as f64 / sp as f64;

            while !stack.is_empty() && *stack.last().unwrap() <= time {
                stack.pop();
            }
            stack.push(time);
        }

        stack.len() as i32
    }
}
