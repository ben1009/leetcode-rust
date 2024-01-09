/// [543] Diameter of Binary Tree
///
/// Given the root of a binary tree, return the length of the diameter of the tree.
/// The diameter of a binary tree is the length of the longest path between any two nodes in a tree.
/// This path may or may not pass through the root. The length of a path between two nodes is
/// represented by the number of edges between them.  
/// <strong class="example">Example 1:
/// <img alt="" src="https://assets.leetcode.com/uploads/2021/03/06/diamtree.jpg" style="width: 292px; height: 302px;" />
/// Input: root = [1,2,3,4,5]
/// Output: 3
/// Explanation: 3 is the length of the path [4,2,1,3] or [5,2,1,3].
///
/// <strong class="example">Example 2:
///
/// Input: root = [1,2]
/// Output: 1
///
///  
/// Constraints:
///
///     The number of nodes in the tree is in the range [1, 10^4].
///     -100 <= Node.val <= 100
pub struct Solution {}
// problem: https://leetcode.com/problems/diameter-of-binary-tree/
// discuss: https://leetcode.com/problems/diameter-of-binary-tree/discuss/?currentPage=1&orderBy=most_votes&query=

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
use std::{cell::RefCell, rc::Rc};

use crate::util::tree::TreeNode;
impl Solution {
    pub fn diameter_of_binary_tree(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let (_, ret) = Self::diameter(root, 0);
        ret
    }

    fn diameter(root: Option<Rc<RefCell<TreeNode>>>, ret: i32) -> (i32, i32) {
        if root.is_none() {
            return (0, ret);
        }
        if root.as_ref().unwrap().borrow().left.is_none()
            && root.as_ref().unwrap().borrow().right.is_none()
        {
            return (0, ret);
        }

        let (l, ret) = Self::diameter(root.as_ref().unwrap().borrow().left.clone(), ret);
        let (r, ret) = Self::diameter(root.as_ref().unwrap().borrow().right.clone(), ret);
        let m = l.max(r) + 1;
        let mut t = l + r;
        if root.as_ref().unwrap().borrow().left.is_some() {
            t += 1;
        }
        if root.as_ref().unwrap().borrow().right.is_some() {
            t += 1;
        }

        (m, ret.max(t))
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;
    use crate::util::tree::to_tree;

    #[test]
    fn test_543() {
        assert_eq!(Solution::diameter_of_binary_tree(tree![1, 2, 3, 4, 5]), 3);
        assert_eq!(Solution::diameter_of_binary_tree(tree![1, 2]), 1);
        assert_eq!(Solution::diameter_of_binary_tree(tree![1]), 0);
    }
}
