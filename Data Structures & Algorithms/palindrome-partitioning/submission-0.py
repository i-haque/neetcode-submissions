class Solution:
    def __init__(self):
        self.partitions = []
        self.res = []

    def partition(self, s: str) -> List[List[str]]:
        res = []
        self.dfs(s, 0)
        return self.res

    def dfs(self, s, i):
        if i == len(s):
            self.res.append(self.partitions[:])
            return
        
        for j in range(i, len(s)):
            if self.is_pallindrome(s[i:j+1]):
                self.partitions.append(s[i:j+1])
                self.dfs(s, j+1)
                self.partitions.pop()

        
    def is_pallindrome(self, substr) -> bool:
        start, end = 0, len(substr)-1

        while start < end:
            if substr[start] != substr[end]:
                return False
            start += 1
            end -= 1
        
        return True