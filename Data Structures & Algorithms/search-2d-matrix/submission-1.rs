impl Solution {
    pub fn search_matrix(matrix: Vec<Vec<i32>>, target: i32) -> bool {
        let (m, n) = (matrix.len() as i32, matrix[0].len() as i32);
        let (mut low, mut high) = (0, m*n);

        while low < high {
            let mid = low + (high - low) / 2;
            if matrix[(mid / n) as usize][(mid % n) as usize] == target {
                return true;
            } else if target < matrix[(mid / n) as usize][(mid % n) as usize] {
                high = mid;
            } else if matrix[(mid / n) as usize][(mid % n) as usize] < target {
                low = mid + 1;
            }
        }

        false
    }
}
