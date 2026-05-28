impl Solution {
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        let n = prices.len();
        if n < 2 {
            return 0;
        }

        let mut max_profit = 0;
        let mut min_price = prices[0];
        
        for price in prices {
            if price < min_price {
                min_price = price;
            } else {
                let profit = price - min_price;
                if max_profit < profit {
                    max_profit = profit;
                }
            }
        }

        max_profit
    }
}
