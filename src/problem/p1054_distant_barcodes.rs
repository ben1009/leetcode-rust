use std::collections::HashMap;

/// [1054] Distant Barcodes
///
/// In a warehouse, there is a row of barcodes, where the i^th barcode is barcodes[i].
/// Rearrange the barcodes so that no two adjacent barcodes are equal. You may return any answer,
/// and it is guaranteed an answer exists.  
/// <strong class="example">Example 1:
/// Input: barcodes = [1,1,1,2,2,2]
/// Output: [2,1,2,1,2,1]
/// <strong class="example">Example 2:
/// Input: barcodes = [1,1,1,1,2,2,3,3]
/// Output: [1,3,1,3,1,2,1,2]
///  
/// Constraints:
///
///     1 <= barcodes.length <= 10000
///     1 <= barcodes[i] <= 10000
pub struct Solution {}

// problem: https://leetcode.com/problems/distant-barcodes/
// discuss: https://leetcode.com/problems/distant-barcodes/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn rearrange_barcodes(barcodes: Vec<i32>) -> Vec<i32> {
        if barcodes.len() < 3 {
            return barcodes;
        }

        let mut map: HashMap<i32, i32> = HashMap::new();
        let mut key = 0;
        for item in &barcodes {
            map.entry(*item).and_modify(|v| *v += 1).or_insert(1);
            if *map.entry(key).or_default() < map[item] {
                key = *item;
            }
        }

        let mut ret = vec![0; barcodes.len()];
        let mut i = 0;
        for _c in 1..=map[&key] {
            ret[i] = key;
            i += 2;
        }

        for (k, v) in &map {
            if *k == key {
                continue;
            }
            for _c in 1..=*v {
                if i >= barcodes.len() {
                    i = 1;
                }
                ret[i] = *k;
                i += 2;
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
    fn test_1054() {
        let a = Solution::rearrange_barcodes(vec![1, 1, 1, 1, 2, 2, 2, 3, 3]);
        assert!(
            [vec![1, 2, 1, 2, 1, 3, 1, 3, 2], vec![
                1, 3, 1, 2, 1, 2, 1, 2, 3
            ]]
            .contains(&a)
        );

        assert_eq!(vec![1, 2], Solution::rearrange_barcodes(vec![1, 2]));
    }
}
