# Definition for a binary tree node.
# class TreeNode:
#     def __init__(self, val=0, left=None, right=None):
#         self.val = val
#         self.left = left
#         self.right = right

class Solution:
    def levelOrder(self, root: Optional[TreeNode]) -> List[List[int]]:
        level_order_traversal = []
        if not root:
            return level_order_traversal

        q = deque([root])
        while q:
            level = []
            for _ in range(len(q)):
                node = q.popleft();
                level.append(node.val)

                if node.left:
                    q.append(node.left)
                if node.right:
                    q.append(node.right)

            level_order_traversal.append(level)
        
        return level_order_traversal
                