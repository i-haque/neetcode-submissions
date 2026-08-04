class Solution:
    def __init__(self):
        self.d = {
            '2': ['a', 'b', 'c'],
            '3': ['d', 'e', 'f'],
            '4': ['g', 'h', 'i'],
            '5': ['j', 'k', 'l'],
            '6': ['m', 'n', 'o'],
            '7': ['p', 'q', 'r', 's'],
            '8': ['t', 'u', 'v'],
            '9': ['w', 'x', 'y', 'z']
        }
        self.temp = []
        self.combinations = []

    def letterCombinations(self, digits: str) -> List[str]:
        if not digits:
            return self.combinations
        
        self.dfs(digits, 0)
        
        return self.combinations
        
    def dfs(self, digits: str, i: int) -> None:
        if i == len(digits):
            self.combinations.append(''.join(self.temp))
            return

        for ch in self.d[digits[i]]:
            self.temp.append(ch)
            self.dfs(digits, i+1)
            self.temp.pop()