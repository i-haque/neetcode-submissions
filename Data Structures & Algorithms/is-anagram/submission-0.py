class Solution:
    def isAnagram(self, s: str, t: str) -> bool:
        freq = dict()
        for ch in s:
            freq[ch] = freq.get(ch, 0) + 1
        for ch in t:
            if ch not in freq:
                return False
            else:
                freq[ch] -= 1
                if freq[ch] == 0:
                    del freq[ch]
        
        return len(freq) == 0