from heapq import heappush, heappop

class MedianFinder:

    def __init__(self):
        self.max_heap = [] # left half
        self.min_heap = [] # right half

    def addNum(self, num: int) -> None:
        # if num > max_heap's top element -> min_heap
        # else -> max_heap
        if self.max_heap and -self.max_heap[0] < num:
            heappush(self.min_heap, num)
        else:
            heappush(self.max_heap, -num)
        
        # balance the heaps
        n1, n2 = len(self.max_heap), len(self.min_heap)

        if n2 - n1 > 1:
            x = heappop(self.min_heap)
            heappush(self.max_heap, -x)
        elif n1 - n2 > 1:
            x = -heappop(self.max_heap)
            heappush(self.min_heap, x)

    def findMedian(self) -> float:
        n1, n2 = len(self.max_heap), len(self.min_heap)

        # bigger heap contains the median
        if n1 > n2:
            return float(-self.max_heap[0])
        elif n2 > n1:
            return float(self.min_heap[0])
        else:
            return (-self.max_heap[0] + self.min_heap[0]) / 2.0
        