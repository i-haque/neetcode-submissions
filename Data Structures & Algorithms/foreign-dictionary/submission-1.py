class Solution:
    def foreignDictionary(self, words: List[str]) -> str:
        # find the alphabetical order and construct a reversed graph
        g = dict()
        for word in words:
            for ch in word:
                g[ch] = set()

        for i in range(len(words)-1):
            w1, w2 = words[i], words[i+1]
            n1, n2 = len(w1), len(w2)
            if n1 > n2 and w1.startswith(w2):
                return ""

            for j in range(min(n1, n2)):
                if w1[j] != w2[j]:
                    g[w2[j]].add(w1[j])
                    break
            
        # apply toposort

        # calculate indegree
        indegree = defaultdict(int)
        for val in g.values():
            for ch in val:
                indegree[ch] += 1

        # get all nodes with indegree 0
        q = deque()
        for ch in g.keys():
            if indegree[ch] == 0:
                q.append(ch)

        # relax edges subsequently
        order = []
        while q:
            node = q.popleft()
            order.append(node)

            for adj in g[node]:
                indegree[adj] -= 1
                if indegree[adj] == 0:
                    q.append(adj)

        if len(order) < len(g):
            return ""
        
        return ''.join(order[::-1])