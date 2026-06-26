# Definition for a binary tree node.
# class TreeNode:
#     def __init__(self, val=0, left=None, right=None):
#         self.val = val
#         self.left = left
#         self.right = right

class Solution:
    def __init__(self):
        self.max_sum = float('-inf')

    def maxPathSum(self, root: Optional[TreeNode]) -> int:
        self.dfs(root)
        return self.max_sum
    
    def dfs(self, root):
        if not root:
            return 0
        
        # if a sub-tree returns a negative value, don't include it
        l = self.dfs(root.left)
        if l < 0:
            l = 0

        r = self.dfs(root.right)
        if r < 0:
            r = 0

        # max path sum with split
        self.max_sum = max(self.max_sum, l + root.val + r)

        # max path sum without split
        return max(l, r) + root.val