class Solution:
    def ladderLength(self, beginWord: str, endWord: str, wordList: List[str]) -> int:
        if beginWord == endWord:
            return 0

        word_list = set(wordList)
        q = deque([(beginWord, 1)])

        while q:
            for _ in range(len(q)):
                word, step = q.popleft()

                for i in range(len(word)):
                    for ch in 'abcdefghijklmnopqrstuvwxyz':
                        new_word = word[:i] + ch + word[i+1:]
                        if (new_word != word) and (new_word in word_list):
                            if new_word == endWord:
                                return step + 1
                            q.append((new_word, step + 1))
                            word_list.remove(new_word)
        
        return 0