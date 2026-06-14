# Definition for singly-linked list.
# class ListNode:
#     def __init__(self, val=0, next=None):
#         self.val = val
#         self.next = next

class Solution:
    def addTwoNumbers(self, l1: Optional[ListNode], l2: Optional[ListNode]) -> Optional[ListNode]:
        carry = 0
        head = ListNode(-1)
        ptr = head 

        while l1 and l2:
            temp = (l1.val + l2.val + carry)
            ptr.next = ListNode(temp % 10)
            ptr = ptr.next
            carry = temp // 10

            l1 = l1.next
            l2 = l2.next
        
        while l1:
            temp = (l1.val + carry)
            ptr.next = ListNode(temp % 10)
            ptr = ptr.next
            carry = temp // 10
            l1 = l1.next
        
        while l2:
            temp = (l2.val + carry)
            ptr.next = ListNode(temp % 10)
            ptr = ptr.next
            carry = temp // 10
            l2 = l2.next
        
        if carry:
            ptr.next = ListNode(carry)
        
        return head.next