class Solution:
    def orangesRotting(self, grid: List[List[int]]) -> int:
        m, n = len(grid), len(grid[0])
        q = deque()
        fresh_oranges = 0

        # collect all the cells with rotten oranges
        for i in range(m):
            for j in range(n):
                if grid[i][j] == 1:
                    fresh_oranges += 1
                elif grid[i][j] == 2:
                    q.append((i, j))
        
        # if no fresh oranges -> early exit
        if fresh_oranges == 0:
            return 0
        
        # using BFS find how many minutes it takes to rot all oranges
        minutes = 0
        while q:
            for _ in range(len(q)):
                i, j = q.popleft()
                for (r, c) in [(i-1, j), (i+1, j), (i, j-1), (i, j+1)]:
                    if 0 <= r < m and 0 <= c < n and grid[r][c] == 1:
                        grid[r][c] = 2
                        fresh_oranges -= 1
                        q.append((r, c))
            
            minutes += 1
        
        return -1 if fresh_oranges else minutes - 1