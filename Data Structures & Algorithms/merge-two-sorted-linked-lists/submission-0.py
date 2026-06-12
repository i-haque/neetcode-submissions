# Definition for singly-linked list.
# class ListNode:
#     def __init__(self, val=0, next=None):
#         self.val = val
#         self.next = next

class Solution:
    def mergeTwoLists(self, list1: Optional[ListNode], list2: Optional[ListNode]) -> Optional[ListNode]:
        new_head = ListNode(-1)
        p0 = new_head

        p1, p2 = list1, list2
        while p1 and p2:
            if p1.val < p2.val:
                p0.next = p1
                p1 = p1.next
            else:
                p0.next = p2
                p2 = p2.next
            p0 = p0.next
        
        while p1:
            p0.next = p1
            p1 = p1.next
            p0 = p0.next
        
        while p2:
            p0.next = p2
            p2 = p2.next
            p0 = p0.next
        
        return new_head.next