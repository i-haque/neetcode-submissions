class Solution:
    def canFinish(self, numCourses: int, prerequisites: List[List[int]]) -> bool:
        # topological sort

        # reverse the graph (here graph is already in reverse order)
        g = [[] for _ in range(numCourses)]
        for [u, v] in prerequisites:
            g[u].append(v)
        
        # calculate indegree of each node
        indegree = [0] * numCourses
        for (node, neighbors) in enumerate(g):
            for neighbor in neighbors:
                indegree[neighbor] += 1

        # store the nodes with indegree 0
        q = deque()
        for (node, degree) in enumerate(indegree):
            if degree == 0:
                q.append(node)

        # using BFS, break the connections and store the nodes with indegree 0
        while q:
            for _ in range(len(q)):
                node = q.popleft()

                for neighbor in g[node]:
                    indegree[neighbor] -= 1
                    if indegree[neighbor] == 0:
                        q.append(neighbor)

        # if all nodes have indegree 0 return True
        for degree in indegree:
            if degree > 0:
                return False
        
        return True