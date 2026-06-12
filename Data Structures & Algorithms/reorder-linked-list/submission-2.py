# Definition for singly-linked list.
# class ListNode:
#     def __init__(self, val=0, next=None):
#         self.val = val
#         self.next = next

class Solution:
    def reorderList(self, head: Optional[ListNode]) -> None:
        if not head or not head.next:
            return

        # get the middle node
        slow = fast = head
        while fast and fast.next:
            slow = slow.next
            fast = fast.next.next
        
        # split the list after mid (slow ptr is the mid) and reverse it
        prev, curr = None, slow.next
        slow.next = None

        while curr:
            n = curr.next
            curr.next = prev
            prev = curr
            curr = n
        
        # now iterate p1 from the head and p2 from mid + 1
        p1, p2 = head, prev
        while p2:
            t1, t2 = p1.next, p2.next
            p1.next = p2
            p2.next = t1
            p1, p2 = t1, t2