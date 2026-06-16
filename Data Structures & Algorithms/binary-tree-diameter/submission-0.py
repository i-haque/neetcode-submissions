# Definition for a binary tree node.
# class TreeNode:
#     def __init__(self, val=0, left=None, right=None):
#         self.val = val
#         self.left = left
#         self.right = right

class Solution:
    def __init__(self):
        self.max_len = 0

    def diameterOfBinaryTree(self, root: Optional[TreeNode]) -> int:
        self.depth(root)
        return self.max_len
    
    def depth(self, root):
        if not root:
            return 0
        
        l = self.depth(root.left)
        r = self.depth(root.right)

        self.max_len = max(self.max_len, l + r)

        return max(l, r) + 1