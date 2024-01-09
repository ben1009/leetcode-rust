/// [56] Merge Intervals
///
/// Given an array of intervals where intervals[i] = [starti, endi], merge all overlapping
/// intervals, and return an array of the non-overlapping intervals that cover all the intervals in
/// the input.  
/// <strong class="example">Example 1:
///
/// Input: intervals = [[1,3],[2,6],[8,10],[15,18]]
/// Output: [[1,6],[8,10],[15,18]]
/// Explanation: Since intervals [1,3] and [2,6] overlap, merge them into [1,6].
///
/// <strong class="example">Example 2:
///
/// Input: intervals = [[1,4],[4,5]]
/// Output: [[1,5]]
/// Explanation: Intervals [1,4] and [4,5] are considered overlapping.
///
///  
/// Constraints:
///
///     1 <= intervals.length <= 10^4
///     intervals[i].length == 2
///     0 <= starti <= endi <= 10^4
pub struct Solution {}

// problem: https://leetcode.com/problems/merge-intervals/
// discuss: https://leetcode.com/problems/merge-intervals/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn merge(intervals: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        if intervals.len() == 1 {
            return intervals;
        }

        let mut intervals = intervals;
        intervals.sort_by(|a, b| a[0].cmp(&b[0]));

        let mut ret = vec![intervals[0].clone()];
        for item in intervals.iter().skip(1) {
            let n = ret.len();
            if ret[n - 1][1] < item[0] {
                ret.push(item.clone());
            } else if ret[n - 1][1] <= item[1] {
                ret[n - 1][1] = item[1];
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
    fn test_56() {
        assert_eq!(
            Solution::merge(vec![vec![2, 6], vec![1, 3], vec![8, 10], vec![15, 18]]),
            vec![vec![1, 6], vec![8, 10], vec![15, 18]]
        );
        assert_eq!(
            Solution::merge(vec![vec![1, 4], vec![4, 5]]),
            vec![vec![1, 5]]
        );
    }
}
