class Solution:
    def canCompleteCircuit(self, gas: List[int], cost: List[int]) -> int:
        n = len(gas)

        total_fuel = total_cost = 0
        for i in range(n):
            total_fuel += gas[i]
            total_cost += cost[i]
        if total_cost > total_fuel:
            return -1

        start_index = 0
        expense = 0
        for i in range(n):
            expense += gas[i] - cost[i]
            if expense < 0:
                expense = 0
                start_index = i+1
        
        return start_index