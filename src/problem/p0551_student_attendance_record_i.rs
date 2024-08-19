// You are given a string s representing an attendance record for a student where each character
// signifies whether the student was absent, late, or present on that day. The record only contains
// the following three characters:

// 'A': Absent.
// 'L': Late.
// 'P': Present.
// The student is eligible for an attendance award if they meet both of the following criteria:

// The student was absent ('A') for strictly fewer than 2 days total.
// The student was never late ('L') for 3 or more consecutive days.
// Return true if the student is eligible for an attendance award, or false otherwise.

// Example 1:

// Input: s = "PPALLP"
// Output: true
// Explanation: The student has fewer than 2 absences and was never late 3 or more consecutive days.
// Example 2:

// Input: s = "PPALLL"
// Output: false
// Explanation: The student was late 3 consecutive days in the last 3 days, so is not eligible for
// the award.

// Constraints:

// 1 <= s.length <= 1000
// s[i] is either 'A', 'L', or 'P'.

// https://leetcode.com/problems/student-attendance-record-i/

pub struct Solution {}

impl Solution {
    pub fn check_record(s: String) -> bool {
        if s.len() < 2 {
            return true;
        }

        let mut a = 0;
        let mut l = 0;
        let s = s.as_bytes();
        for i in 0..s.len() {
            if s[i] == b'L' {
                if i == 0 || s[i - 1] != b'L' {
                    l = 1;
                } else {
                    l += 1;
                    if l == 3 {
                        return false;
                    }
                }
            } else if s[i] == b'A' {
                a += 1;
            }
        }

        a < 2
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_0551() {
        assert!(Solution::check_record("PPALLP".to_string()));
        assert!(!Solution::check_record("PPALLL".to_string()));
        assert!(Solution::check_record("A".to_string()));
    }
}
