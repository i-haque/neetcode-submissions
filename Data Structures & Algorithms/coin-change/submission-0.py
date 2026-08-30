class Solution:
    def __init__(self):
        self.dp = dict()

    def coinChange(self, coins: List[int], amount: int) -> int:
        min_coins = self.f(coins, 0, amount)
        if min_coins == float('inf'):
            return -1
        return min_coins
        
    def f(self, coins, i, amount):
        if i == len(coins):
            if amount == 0:
                return 0
            return float('inf')

        if (i, amount) in self.dp:
            return self.dp[(i, amount)]
        
        curr_coins = 0
        if coins[i] <= amount:
            curr_coins = min(1 + self.f(coins, i, amount - coins[i]), self.f(coins, i+1, amount))
        else:
            curr_coins = self.f(coins, i+1, amount)
        
        self.dp[(i, amount)] = curr_coins
        return curr_coins