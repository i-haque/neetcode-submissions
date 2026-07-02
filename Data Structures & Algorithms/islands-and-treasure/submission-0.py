class Solution:
    def islandsAndTreasure(self, grid: List[List[int]]) -> None:
        INF = 2147483647
        m, n = len(grid), len(grid[0])
        q = deque()

        for i in range(m):
            for j in range(n):
                if grid[i][j] == 0:
                    q.append((i, j, 0))
        
        while q:
            for _ in range(len(q)):
                i, j, dist = q.popleft()

                for (r, c) in [(i-1, j), (i+1, j), (i, j-1), (i, j+1)]:
                    if 0 <= r < m and 0 <= c < n and grid[r][c] == INF:
                        grid[r][c] = dist + 1
                        q.append((r, c, dist + 1))