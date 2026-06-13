# Definition for singly-linked list.
# class ListNode:
#     def __init__(self, val=0, next=None):
#         self.val = val
#         self.next = next

class Solution:
    def removeNthFromEnd(self, head: Optional[ListNode], n: int) -> Optional[ListNode]:
        count = 0
        ptr = head
        while ptr:
            ptr = ptr.next
            count += 1
        
        k = count - n
        dummy = ListNode(-1)
        dummy.next = head
        ptr = dummy
        while k > 0:
            ptr = ptr.next
            k -= 1
        ptr.next = ptr.next.next
        
        return dummy.next