class Solution:
    def exist(self, board: List[List[str]], word: str) -> bool:
        m, n = len(board), len(board[0])
        vis = [[False for _ in range(n)] for _ in range(m)]
        q = deque()

        for i in range(m):
            for j in range(n):
                if board[i][j] == word[0]:
                    if self.dfs(board, i, j, word, 0, vis):
                        return True
        
        return False
    
    def dfs(self, board, row, col, word, index, vis):
        if index == len(word) - 1:
            return True

        vis[row][col] = True

        for (r, c) in [(row - 1, col), (row + 1, col), (row, col - 1), (row, col + 1)]:
            if 0 <= r < len(board) and 0 <= c < len(board[0]) and not vis[r][c]:
                if index + 1 < len(word) and word[index + 1] == board[r][c]:
                    if self.dfs(board, r, c, word, index + 1, vis):
                        return True

        vis[row][col] = False

        return False