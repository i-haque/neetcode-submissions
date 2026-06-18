# Definition for a binary tree node.
# class TreeNode:
#     def __init__(self, val=0, left=None, right=None):
#         self.val = val
#         self.left = left
#         self.right = right

class Solution:
    def __init__(self):
        self.good_nodes = 0

    def goodNodes(self, root: TreeNode) -> int:
        self.f(root, float('-inf'))
        return self.good_nodes
        
    def f(self, root, max_val):
        if not root:
            return
        
        if root.val >= max_val:
            self.good_nodes += 1
        max_val = max(max_val, root.val)

        self.f(root.left, max_val)
        self.f(root.right, max_val)