// Given an array of integers temperatures represents the daily temperatures, return an array answer
// such that answer[i] is the number of days you have to wait after the ith day to get a warmer
// temperature. If there is no future day for which this is possible, keep answer[i] == 0 instead.

// Example 1:

// Input: temperatures = [73,74,75,71,69,72,76,73]
// Output: [1,1,4,2,1,1,0,0]
// Example 2:

// Input: temperatures = [30,40,50,60]
// Output: [1,1,1,0]
// Example 3:

// Input: temperatures = [30,60,90]
// Output: [1,1,0]

// Constraints:

// 1 <= temperatures.length <= 105
// 30 <= temperatures[i] <= 100

pub struct Solution {}

// problem: https://leetcode.com/problems/daily-temperatures/
// discuss: https://leetcode.com/problems/daily-temperatures/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn daily_temperatures(temperatures: Vec<i32>) -> Vec<i32> {
        if temperatures.len() == 1 {
            return vec![0];
        }

        let mut ret = vec![0; temperatures.len()];
        let mut stack = vec![];
        for i in 0..temperatures.len() {
            while !stack.is_empty() && temperatures[stack[stack.len() - 1]] < temperatures[i] {
                let j = stack.pop().unwrap();
                ret[j] = (i - j) as i32;
            }
            stack.push(i);
        }

        ret
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_0739() {
        assert_eq!(
            Solution::daily_temperatures(vec![73, 74, 75, 71, 69, 72, 76, 73]),
            vec![1, 1, 4, 2, 1, 1, 0, 0]
        );
        assert_eq!(
            Solution::daily_temperatures(vec![30, 40, 50, 60]),
            vec![1, 1, 1, 0]
        );
        assert_eq!(
            Solution::daily_temperatures(vec![30, 60, 90]),
            vec![1, 1, 0]
        );
    }
}
