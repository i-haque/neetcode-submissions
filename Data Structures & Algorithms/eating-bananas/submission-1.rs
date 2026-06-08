impl Solution {
    pub fn min_eating_speed(piles: Vec<i32>, h: i32) -> i32 {
        let (mut low, mut high) = (1, *piles.iter().max().unwrap() + 1);

        while low < high {
            let rate = low + (high - low) / 2;

            let mut time = 0;
            for pile in &piles {
                time += *pile / rate;
                if *pile % rate != 0 {
                    time += 1;
                }
            }

            if time > h {
                low = rate + 1;
            } else {
                high = rate;
            }
        }

        high
    }
}
