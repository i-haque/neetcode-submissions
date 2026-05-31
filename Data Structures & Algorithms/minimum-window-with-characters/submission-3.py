class Solution:
    def minWindow(self, s: str, t: str) -> str:
        n1, n2 = len(s), len(t)
        if n2 == 0 or n2 > n1:
            return ""

        freq_t = defaultdict(int)
        for ch in t:
            freq_t[ch] += 1

        res = [-1, -1]
        length = float('inf')
        have, need = 0, len(freq_t)

        freq_s = defaultdict(int)
        j = 0
        for i in range(n1):
            if s[i] in freq_t:
                freq_s[s[i]] += 1
                if freq_s[s[i]] == freq_t[s[i]]:
                    have += 1

            while have == need:
                if (i-j+1) < length:
                    res = [j, i]
                    length = (i-j+1)
                
                if (s[j] in freq_t):
                    freq_s[s[j]] -= 1
                    if freq_s[s[j]] < freq_t[s[j]]:
                        have -= 1

                j += 1
        
        start, end = res
        
        return s[start: end+1] if length != float('inf') else ""
            
