# Definition for a binary tree node.
# class TreeNode:
#     def __init__(self, val=0, left=None, right=None):
#         self.val = val
#         self.left = left
#         self.right = right

class Solution:
    def rightSideView(self, root: Optional[TreeNode]) -> List[int]:
        right_view = []
        if not root:
            return right_view

        level_map = dict()

        q = deque([(root, 0)])
        while q:
            for _ in range(len(q)):
                node, level = q.popleft()
                level_map[level] = node.val

                if node.left:
                    q.append((node.left, level + 1))
                if node.right:
                    q.append((node.right, level + 1))
            
        for key in sorted(level_map.keys()):
            right_view.append(level_map[key])

        return right_view