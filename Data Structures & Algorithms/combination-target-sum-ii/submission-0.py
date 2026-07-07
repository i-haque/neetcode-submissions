class Solution:
    def __init__(self):
        self.combinations = []

    def combinationSum2(self, candidates: List[int], target: int) -> List[List[int]]:
        candidates.sort()
        self.f(candidates, target, 0, 0, [])
        return self.combinations
        
    def f(self, candidates, target, i, curr_sum, temp):
        if curr_sum == target:
            self.combinations.append(temp[:])
            return

        for index in range(i, len(candidates)):
            if index > i and candidates[index-1] == candidates[index]:
                continue
        
            # include
            curr_sum += candidates[index]
            temp.append(candidates[index])

            if curr_sum <= target:
                self.f(candidates, target, index+1, curr_sum, temp)

            # exclude
            curr_sum -= candidates[index]
            temp.pop()