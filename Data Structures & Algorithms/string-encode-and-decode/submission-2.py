class Solution:
    def encode(self, strs: List[str]) -> str:
        encoded = ""
        for s in strs:
            encoded += str(len(s)) + '#' + s
        return encoded

    def decode(self, s: str) -> List[str]:
        res = []
        if len(s) == 0:
            return res
            
        i = 0
        while i < len(s):
            length = ''
            while s[i] != '#':
                length += s[i]
                i += 1
            word = ""
            for j in range(i+1, i+1+int(length)):
                word += s[j]
            res.append(word)
            i += int(length)+1
        return res