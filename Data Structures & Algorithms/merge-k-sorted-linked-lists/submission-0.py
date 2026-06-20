# Definition for singly-linked list.
# class ListNode:
#     def __init__(self, val=0, next=None):
#         self.val = val
#         self.next = next

class Solution:    
    def mergeKLists(self, lists: List[Optional[ListNode]]) -> Optional[ListNode]:
        if len(lists) == 0:
            return None

        while len(lists) > 1:
            n = len(lists)
            merged_lists = []

            for i in range(0, n, 2):
                l1 = lists[i]
                l2 = lists[i + 1] if (i + 1) < n else None
                merged_list = self.merge_two_lists(l1, l2)
                merged_lists.append(merged_list)
            
            lists = merged_lists
        
        return lists[0]
        
    def merge_two_lists(self, l1, l2):
        dummy = ptr = ListNode(-1)
        
        while l1 and l2:
            if l1.val < l2.val:
                ptr.next = l1
                l1 = l1.next
            else:
                ptr.next = l2
                l2 = l2.next
            ptr = ptr.next
        
        while l1:
            ptr.next = l1
            l1 = l1.next
            ptr = ptr.next
        
        while l2:
            ptr.next = l2
            l2 = l2.next
            ptr = ptr.next
        
        return dummy.next