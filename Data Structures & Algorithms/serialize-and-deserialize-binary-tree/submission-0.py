# Definition for a binary tree node.
# class TreeNode:
#     def __init__(self, val=0, left=None, right=None):
#         self.val = val
#         self.left = left
#         self.right = right

class Codec:
    
    # Encodes a tree to a single string.
    def serialize(self, root: Optional[TreeNode]) -> str:
        if not root:
            return ''

        level_order = []
        q = deque([root])

        while q:
            for _ in range(len(q)):
                node = q.popleft()
                if node:
                    level_order.append(str(node.val))
                    q.append(node.left)
                    q.append(node.right)
                else:
                    level_order.append('#')
        
        return ','.join(level_order)
        
    # Decodes your encoded data to tree.
    def deserialize(self, data: str) -> Optional[TreeNode]:
        if not data:
            return None

        level_order = data.split(',')
        root = TreeNode(int(level_order[0]))
        q = deque([root])

        i = 1
        while q:
            for _ in range(len(q)):
                node = q.popleft()

                # attach left node
                if level_order[i] != '#':
                    left_node = TreeNode(int(level_order[i]))
                    node.left = left_node
                    q.append(left_node)
                i += 1

                # attach right node
                if level_order[i] != '#':
                    right_node = TreeNode(int(level_order[i]))
                    node.right = right_node
                    q.append(right_node)
                i += 1
        
        return root