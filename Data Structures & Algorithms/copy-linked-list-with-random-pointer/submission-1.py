"""
# Definition for a Node.
class Node:
    def __init__(self, x: int, next: 'Node' = None, random: 'Node' = None):
        self.val = int(x)
        self.next = next
        self.random = random
"""

class Solution:
    def copyRandomList(self, head: 'Optional[Node]') -> 'Optional[Node]':
        map = dict()

        ptr = head
        while ptr:
            map[ptr] = Node(ptr.val)
            ptr = ptr.next
        
        ptr = head
        while ptr:
            map[ptr].next = map[ptr.next] if ptr.next else None
            map[ptr].random = map[ptr.random] if ptr.random else None
            ptr = ptr.next
        
        return map[head] if head else None