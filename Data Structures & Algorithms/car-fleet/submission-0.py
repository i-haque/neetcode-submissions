class Solution:
    def carFleet(self, target: int, position: List[int], speed: List[int]) -> int:
        cars = [(pos, sp) for (pos, sp) in zip(position, speed)]
        cars.sort(key=lambda x: x[0])
        
        stack = []
        for (pos, sp) in cars:
            time = (target - pos) / sp

            while stack and stack[-1] <= (time):
                stack.pop()
            stack.append(time)
        
        return len(stack)