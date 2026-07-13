class Solution:
    def __init__(self):
        self.res = []

    def generateParenthesis(self, n: int) -> List[str]:
        self.f(n, 0, 0, [])
        return self.res
        
    def f(self, n, op, cl, temp):
        if op == cl == n:
            self.res.append(''.join(temp[:]))
            return
        
        if op < n:
            temp.append('(')
            self.f(n, op + 1, cl, temp)
            temp.pop()

        if op > cl:
            temp.append(')')
            self.f(n, op, cl + 1, temp)
            temp.pop()
        