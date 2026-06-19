# Definition for a binary tree node.
# class TreeNode:
#     def __init__(self, val=0, left=None, right=None):
#         self.val = val
#         self.left = left
#         self.right = right

class Solution:
    def isValidBST(self, root: Optional[TreeNode]) -> bool:
        return self.f(root, float('-inf'), float('inf'))
        
    def f(self, root, l, r):
        if not root:
            return True
        
        return l < root.val < r and self.f(root.left, min(l, root.val), max(l, root.val)) and self.f(root.right, min(root.val, r), max(root.val, r))