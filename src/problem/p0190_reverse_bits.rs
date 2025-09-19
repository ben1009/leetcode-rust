// Reverse bits of a given 32 bits signed integer.

// Example 1:

// Input: n = 43261596

// Output: 964176192

// Explanation:

// Integer	Binary
// 43261596	00000010100101000001111010011100
// 964176192	00111001011110000010100101000000
// Example 2:

// Input: n = 2147483644

// Output: 1073741822

// Explanation:

// Integer	Binary
// 2147483644	01111111111111111111111111111100
// 1073741822	00111111111111111111111111111110

// Constraints:

// 0 <= n <= 231 - 2
// n is even.

// Follow up: If this function is called many times, how would you optimize it?

pub struct Solution;

impl Solution {
    pub fn reverse_bits(n: i32) -> i32 {
        n.reverse_bits()
    }

    pub fn reverse_bits_1(n: i32) -> i32 {
        if n == 0 {
            return n;
        }

        let mut ret = 0;
        let mut n = n;
        for _i in 0..32 {
            ret <<= 1;
            // n&1 get the last bit, e.g. n=0b101, n&0b001=1; n=0b100, n&0b001=0
            // ret|=n&1 set the last bit to ret, e.g. ret=0b110, n&0b001=1, ret|0b001=0b111;
            ret |= n & 1;
            n >>= 1;
        }

        ret
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_190() {
        assert_eq!(Solution::reverse_bits(43261596), 964176192);
        assert_eq!(Solution::reverse_bits(2147483644), 1073741822);
        assert_eq!(Solution::reverse_bits(0), 0);
        assert_eq!(Solution::reverse_bits_1(43261596), 964176192);
        assert_eq!(Solution::reverse_bits_1(2147483644), 1073741822);
        assert_eq!(Solution::reverse_bits_1(0), 0);
    }
}
