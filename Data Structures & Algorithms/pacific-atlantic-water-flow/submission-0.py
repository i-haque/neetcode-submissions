class Solution:
    def pacificAtlantic(self, heights: List[List[int]]) -> List[List[int]]:
        m, n = len(heights), len(heights[0])
        pac, atl = set(), set()

        # cells reachable from pacific co-ordinates
        for c in range(n):
            self.dfs(heights, 0, c, pac)
        
        for r in range(m):
            self.dfs(heights, r, 0, pac)

        # cells reachable from atlantic co-ordinates
        for c in range(n):
            self.dfs(heights, m-1, c, atl)
        
        for r in range(m):
            self.dfs(heights, r, n-1, atl)
        
        res = []
        for (r, c) in pac:
            if (r, c) in atl:
                res.append([r, c])
        
        return res
    
    def dfs(self, heights, i, j, vis):
        vis.add((i, j))

        for (r, c) in [(i-1, j), (i+1, j), (i, j-1), (i, j+1)]:
            if 0 <= r < len(heights) and 0 <= c < len(heights[0]):
                if (r, c) not in vis and heights[r][c] >= heights[i][j]:
                    self.dfs(heights, r, c, vis)