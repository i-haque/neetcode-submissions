class Solution:
    def findOrder(self, numCourses: int, prerequisites: List[List[int]]) -> List[int]:
        # topological sort

        # reverse the graph (in this case graph is already reversed)
        g = [[] for _ in range(numCourses)]
        for [u, v] in prerequisites:
            g[u].append(v)

        # calculate indegree of each node
        indegree = [0] * numCourses
        for neighbors in g:
            for neighbor in neighbors:
                indegree[neighbor] += 1

        # store all nodes with indegree 0, relax their edges and collect relaxed edges with indegree 0 using BFS
        q = deque()
        for (node, degree) in enumerate(indegree):
            if degree == 0:
                q.append(node)
        
        course_order = []
        while q:
            for _ in range(len(q)):
                node = q.popleft()
                course_order.append(node)

                for neighbor in g[node]:
                    indegree[neighbor] -= 1
                    if indegree[neighbor] == 0:
                        q.append(neighbor)
            
        # if all nodes are reached, return a reversed list or else an empty list
        return course_order[::-1] if len(course_order) == numCourses else []