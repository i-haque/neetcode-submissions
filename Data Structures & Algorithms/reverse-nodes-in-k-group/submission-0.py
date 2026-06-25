# Definition for singly-linked list.
# class ListNode:
#     def __init__(self, val=0, next=None):
#         self.val = val
#         self.next = next

class Solution:
    def reverseKGroup(self, head: Optional[ListNode], k: int) -> Optional[ListNode]:
        dummy = ListNode(-1, head)
        prev_group = dummy

        while True:
            kth = self.get_kth_node(prev_group, k)
            if not kth:
                break
            
            # reversing group
            prev, curr = kth.next, prev_group.next
            for _ in range(k):
                n = curr.next
                curr.next = prev
                prev = curr
                curr = n
            
            curr_group_end = prev_group.next
            prev_group.next = kth
            prev_group = curr_group_end
        
        return dummy.next
    
    def get_kth_node(self, ptr, k):
        while ptr and k > 0:
            ptr = ptr.next
            k -= 1
        
        return ptr