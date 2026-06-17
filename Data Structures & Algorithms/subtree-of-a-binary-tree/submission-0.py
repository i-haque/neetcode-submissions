# Definition for a binary tree node.
# class TreeNode:
#     def __init__(self, val=0, left=None, right=None):
#         self.val = val
#         self.left = left
#         self.right = right

class Solution:   
    def isSubtree(self, root: Optional[TreeNode], subRoot: Optional[TreeNode]) -> bool:
        q = deque([root])
        while q:
            r = q.popleft()
            if self.f(r, subRoot):
                return True
            if r.left:
                q.append(r.left)
            if r.right:
                q.append(r.right)
        
        return False
        
    def f(self, r1, r2) -> bool:
        if not r1 and not r2:
            return True
        if not r1 or not r2:
            return False
        
        return r1.val == r2.val and self.f(r1.left, r2.left) and self.f(r1.right, r2.right)