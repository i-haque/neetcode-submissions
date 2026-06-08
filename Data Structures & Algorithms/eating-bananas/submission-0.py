class Solution:
    def minEatingSpeed(self, piles: List[int], h: int) -> int:
        low, high = 1, max(piles) + 1

        while low < high:
            rate = low + (high - low) // 2

            time = 0
            for pile in piles:
                q, r = divmod(pile, rate)
                time += q
                if r:
                    time += 1
            
            if time > h:
                low = rate + 1
            else:
                high = rate
        
        return high
