"""
# Definition for a Node.
class Node:
    def __init__(self, val = 0, neighbors = None):
        self.val = val
        self.neighbors = neighbors if neighbors is not None else []
"""

class Solution:
    def cloneGraph(self, node: Optional['Node']) -> Optional['Node']:
        if not node:
            return None
        
        vis = dict()   # {node -> node}

        q = deque([node])
        vis[node] = Node(node.val)

        while q:
            curr_node = q.popleft()
            
            for adj_node in curr_node.neighbors:
                if adj_node not in vis:
                    q.append(adj_node)
                    vis[adj_node] = Node(adj_node.val)

                vis[curr_node].neighbors.append(vis[adj_node])
        
        return vis[node]
        