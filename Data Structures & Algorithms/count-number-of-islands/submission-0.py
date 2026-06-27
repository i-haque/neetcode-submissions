class Solution:
    def numIslands(self, grid: List[List[str]]) -> int:
        m, n = len(grid), len(grid[0])
        vis = [[False for _ in range(n)] for _ in range(m)]

        islands = 0

        for i in range(m):
            for j in range(n):
                if grid[i][j] == '1' and vis[i][j] == False:
                    q = deque([(i, j)])
                    vis[i][j] = True

                    while q:
                        for _ in range(len(q)):
                            r, c = q.popleft()
                            for (row, col) in [(r + 1, c), (r - 1, c), (r, c + 1), (r, c - 1)]:
                                if 0 <= row < m and 0 <= col < n and grid[row][col] == '1' and vis[row][col] == False:
                                    q.append((row, col))
                                    vis[row][col] = True
                    
                    islands += 1
        
        return islands
                    