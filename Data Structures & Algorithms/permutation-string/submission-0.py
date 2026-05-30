class Solution:
    def checkInclusion(self, s1: str, s2: str) -> bool:
        n1, n2 = len(s1), len(s2)
        if n1 > n2:
            return False

        d1 = defaultdict(int)
        for ch in s1:
            d1[ch] += 1

        d2 = defaultdict(int)
        j = 0
        for i in range(n2):
            d2[s2[i]] += 1

            if i-j+1 == n1:
                if d1 == d2:
                    return True

                d2[s2[j]] -= 1
                if d2[s2[j]] == 0:
                    del d2[s2[j]]

                j += 1
        
        return False