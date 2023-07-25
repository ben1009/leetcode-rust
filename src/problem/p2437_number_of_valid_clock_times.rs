/// [2437] Number of Valid Clock Times
///
/// You are given a string of length 5 called time, representing the current time on a digital clock
/// in the format "hh:mm". The earliest possible time is "00:00" and the latest possible time is
/// "23:59". In the string time, the digits represented by the ? symbol are unknown, and must be
/// replaced with a digit from 0 to 9. Return an integer answer, the number of valid clock times
/// that can be created by replacing every ? with a digit from 0 to 9.  
/// <strong class="example">Example 1:
///
/// Input: time = "?5:00"
/// Output: 2
/// Explanation: We can replace the ? with either a 0 or 1, producing "05:00" or "15:00". Note that
/// we cannot replace it with a 2, since the time "25:00" is invalid. In total, we have two choices.
///
/// <strong class="example">Example 2:
///
/// Input: time = "0?:0?"
/// Output: 100
/// Explanation: Each ? can be replaced by any digit from 0 to 9, so we have 100 total choices.
///
/// <strong class="example">Example 3:
///
/// Input: time = "??:??"
/// Output: 1440
/// Explanation: There are 24 possible choices for the hours, and 60 possible choices for the
/// minutes. In total, we have 24 * 60 = 1440 choices.
///
///  
/// Constraints:
///
///     time is a valid string of length 5 in the format "hh:mm".
///     "00" <= hh <= "23"
///     "00" <= mm <= "59"
///     Some of the digits might be replaced with '?' and need to be replaced with digits from 0 to
/// 9.
///
pub struct Solution {}

// problem: https://leetcode.com/problems/number-of-valid-clock-times/
// discuss: https://leetcode.com/problems/number-of-valid-clock-times/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn count_time(time: String) -> i32 {
        let time = time.as_bytes();
        let mut ret = 1;
        if time[4] == b'?' {
            ret *= 10;
        }
        if time[3] == b'?' {
            ret *= 6;
        }
        if time[0] == b'?' && time[1] == b'?' {
            ret *= 24;
        }
        if time[0] != b'?' && time[1] == b'?' {
            if time[0] - b'0' < 2 {
                ret *= 10;
            } else {
                ret *= 4;
            }
        }
        if time[0] == b'?' && time[1] != b'?' {
            if time[1] - b'0' < 4 {
                ret *= 3;
            } else {
                ret *= 2;
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
    fn test_2437() {
        assert_eq!(Solution::count_time("?5:00".to_string()), 2);
        assert_eq!(Solution::count_time("0?:0?".to_string()), 100);
        assert_eq!(Solution::count_time("??:??".to_string()), 1440);
    }
}
