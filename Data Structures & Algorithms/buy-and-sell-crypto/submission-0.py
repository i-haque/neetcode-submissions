class Solution:
    def maxProfit(self, prices: List[int]) -> int:
        n = len(prices)
        max_profit = 0
        if n < 2:
            return max_profit

        buy, sell = 0, 1
        while sell < n:
            if prices[sell] < prices[buy]:
                buy = sell
            else:
                max_profit = max(max_profit, prices[sell] - prices[buy])
            sell += 1
        
        return max_profit