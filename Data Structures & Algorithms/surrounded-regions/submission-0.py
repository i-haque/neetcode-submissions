class Solution:
    def solve(self, board: List[List[str]]) -> None:
        m, n = len(board), len(board[0])
        q = deque()
        vis = [[False for _ in range(n)] for _ in range(m)]

        # collect all the boundary cells having 'O'
        for j in range(n):
            if board[0][j] == 'O':
                q.append((0, j))
                vis[0][j] = True
            if board[m-1][j] == 'O':
                q.append((m-1, j))
                vis[m-1][j] = True
        
        for i in range(m):
            if board[i][0] == 'O':
                q.append((i, 0))
                vis[i][0] = True
            if board[i][n-1] == 'O':
                q.append((i, n-1))
                vis[i][n-1] = True
        
        # mark all the cells that can be reached through the boundaries
        while q:
            for _ in range(len(q)):
                i, j = q.popleft()
                for (r, c) in [(i-1, j), (i+1, j), (i, j-1), (i, j+1)]:
                    if 0 <= r < m and 0 <= c < n:
                        if board[r][c] == 'O' and vis[r][c] == False:
                            q.append((r, c))
                            vis[r][c] = True
        
        # mark the reached cells by 'O' and the rest by 'X'
        for i in range(m):
            for j in range(n):
                if vis[i][j]:
                    board[i][j] = 'O'
                else:
                    board[i][j] = 'X'
