# Definition for singly-linked list.
# class ListNode:
#     def __init__(self, val=0, next=None):
#         self.val = val
#         self.next = next

class Solution:
    def reorderList(self, head: Optional[ListNode]) -> None:
        if not head.next:
            return
        # get the middle node
        slow = fast = head
        while fast and fast.next:
            slow = slow.next
            fast = fast.next.next
        
        # reverse the list after mid (slow ptr is the mid) and attach it to mid
        prev, curr = None, slow.next
        while curr:
            n = curr.next
            curr.next = prev
            prev = curr
            curr = n
        
        slow.next = prev
        
        # now iterate p1 from the head and p2 from mid + 1
        p0, p1, p2 = head, head.next, slow.next
        turn = 0
        while p2:
            if turn == 0:
                p0.next = p2
                p2 = p2.next
                turn = 1 - turn
            else:
                p0.next = p1
                p1 = p1.next
                turn = 1 - turn
            p0 = p0.next
        
        while p1 != slow:
            p0.next = p1
            p1 = p1.next
            p0 = p0.next
        
        p0.next = p1
        p0 = p0.next
        p0.next = None