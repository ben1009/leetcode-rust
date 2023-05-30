/**
 * [95] Unique Binary Search Trees II
 *
 * Given an integer n, return all the structurally unique BST's (binary search trees), which has exactly n nodes of unique values from 1 to n. Return the answer in any order.
 *  
 * <strong class="example">Example 1:
 * <img alt="" src="https://assets.leetcode.com/uploads/2021/01/18/uniquebstn3.jpg" style="width: 600px; height: 148px;" />
 * Input: n = 3
 * Output: [[1,null,2,null,3],[1,null,3,2],[2,1,3],[3,1,null,null,2],[3,2,null,1]]
 *
 * <strong class="example">Example 2:
 *
 * Input: n = 1
 * Output: [[1]]
 *
 *  
 * Constraints:
 *
 *     1 <= n <= 8
 *
 */
pub struct Solution {}
use crate::util::tree::{to_tree, TreeNode};

// problem: https://leetcode.com/problems/unique-binary-search-trees-ii/
// discuss: https://leetcode.com/problems/unique-binary-search-trees-ii/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

// Definition for a binary tree node.
// #[derive(Debug, PartialEq, Eq)]
// pub struct TreeNode {
//   pub val: i32,
//   pub left: Option<Rc<RefCell<TreeNode>>>,
//   pub right: Option<Rc<RefCell<TreeNode>>>,
// }
//
// impl TreeNode {
//   #[inline]
//   pub fn new(val: i32) -> Self {
//     TreeNode {
//       val,
//       left: None,
//       right: None
//     }
//   }
// }
use std::cell::RefCell;
use std::rc::Rc;
use std::vec;
impl Solution {
    pub fn generate_trees(n: i32) -> Vec<Option<Rc<RefCell<TreeNode>>>> {
        Self::gen(1, n)
    }

    fn gen(i: i32, j: i32) -> Vec<Option<Rc<RefCell<TreeNode>>>> {
        if i > j {
            return vec![None];
        }

        let mut ret = vec![];
        for k in i..=j {
            let left = Self::gen(i, k - 1);
            let right = Self::gen(k + 1, j);
            for l in &left {
                for r in &right {
                    let mut node = TreeNode::new(k);
                    node.left = l.clone();
                    node.right = r.clone();
                    ret.push(Some(Rc::new(RefCell::new(node))));
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
    fn test_95() {
        assert_eq!(
            Solution::generate_trees(3),
            vec![
                tree![1, null, 2, null, 3],
                tree![1, null, 3, 2],
                tree![2, 1, 3],
                tree![3, 1, null, null, 2],
                tree![3, 2, null, 1]
            ]
        );
    }
}
