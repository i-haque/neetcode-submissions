# Definition for a binary tree node.
# class TreeNode:
#     def __init__(self, val=0, left=None, right=None):
#         self.val = val
#         self.left = left
#         self.right = right

class Solution:
    def __init__(self):
        self.count = 0
        self.res = -1

    def kthSmallest(self, root: Optional[TreeNode], k: int) -> int:
        self.f(root, k)
        return self.res
        
    def f(self, root, k):
        if not root:
            return

        self.f(root.left, k)

        self.count += 1
        if self.count == k:
            self.res = root.val
            return
        
        self.f(root.right, k)