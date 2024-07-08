// Given a 0-indexed m x n integer matrix matrix, create a new 0-indexed matrix called answer. Make
// answer equal to matrix, then replace each element with the value -1 with the maximum element in
// its respective column.

// Return the matrix answer.

// Example 1:

// Input: matrix = [[1,2,-1],[4,-1,6],[7,8,9]]
// Output: [[1,2,9],[4,8,6],[7,8,9]]
// Explanation: The diagram above shows the elements that are changed (in blue).
// - We replace the value in the cell [1][1] with the maximum value in the column 1, that is 8.
// - We replace the value in the cell [0][2] with the maximum value in the column 2, that is 9.
// Example 2:

// Input: matrix = [[3,-1],[5,2]]
// Output: [[3,2],[5,2]]
// Explanation: The diagram above shows the elements that are changed (in blue).

// Constraints:

// m == matrix.length
// n == matrix[i].length
// 2 <= m, n <= 50
// -1 <= matrix[i][j] <= 100
// The input is generated such that each column contains at least one non-negative integer.

// https://leetcode.com/problems/modify-the-matrix/
// https://leetcode.com/problems/modify-the-matrix/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here
pub struct Solution {}

#[allow(clippy::needless_range_loop)]
impl Solution {
    pub fn modified_matrix(matrix: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let m = matrix.len();
        let n = matrix[0].len();
        let mut ret = vec![vec![0; n]; m];
        let mut max = vec![0; n];
        for i in 0..m {
            for j in 0..n {
                ret[i][j] = matrix[i][j];
                max[j] = std::cmp::max(max[j], matrix[i][j]);
            }
        }

        for i in 0..m {
            for j in 0..n {
                if ret[i][j] == -1 {
                    ret[i][j] = max[j];
                }
            }
        }

        ret
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_3033() {
        assert_eq!(
            Solution::modified_matrix(vec![vec![1, 2, -1], vec![4, -1, 6], vec![7, 8, 9]]),
            vec![vec![1, 2, 9], vec![4, 8, 6], vec![7, 8, 9]]
        );
        assert_eq!(
            Solution::modified_matrix(vec![vec![3, -1], vec![5, 2]]),
            vec![vec![3, 2], vec![5, 2]]
        );
    }
}
