class Solution:
    def characterReplacement(self, s: str, k: int) -> int:
        freq = defaultdict(int)
        max_freq = 0
        res = 0
        
        j = 0
        for i in range(len(s)):
            freq[s[i]] += 1
            max_freq = max(max_freq, freq[s[i]])

            while (i-j+1) - max_freq > k:
                freq[s[j]] -= 1
                j += 1
            
            res = max(res, i-j+1)
        
        return res
