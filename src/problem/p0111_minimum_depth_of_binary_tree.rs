/// [111] Minimum Depth of Binary Tree
///
/// Given a binary tree, find its minimum depth.
/// The minimum depth is the number of nodes along the shortest path from the root node down to the
/// nearest leaf node. Note: A leaf is a node with no children.
///  
/// <strong class="example">Example 1:
/// <img alt="" src="https://assets.leetcode.com/uploads/2020/10/12/ex_depth.jpg" style="width: 432px; height: 302px;" />
/// Input: root = [3,9,20,null,null,15,7]
/// Output: 2
///
/// <strong class="example">Example 2:
///
/// Input: root = [2,null,3,null,4,null,5,null,6]
/// Output: 5
///
///  
/// Constraints:
///
///     The number of nodes in the tree is in the range [0, 10^5].
///     -1000 <= Node.val <= 1000
pub struct Solution {}
// problem: https://leetcode.com/problems/minimum-depth-of-binary-tree/
// discuss: https://leetcode.com/problems/minimum-depth-of-binary-tree/discuss/?currentPage=1&orderBy=most_votes&query=

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
use std::{cell::RefCell, collections::VecDeque, rc::Rc};

use crate::util::tree::TreeNode;

impl Solution {
    pub fn min_depth(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        if root.is_none() {
            return 0;
        }

        let mut root = root;
        let mut ret = 1;
        let mut queue = VecDeque::from([(root, ret)]);
        while !queue.is_empty() {
            (root, ret) = queue.pop_front().unwrap();
            if root.as_ref().unwrap().borrow().left.is_none()
                && root.as_ref().unwrap().borrow().right.is_none()
            {
                break;
            }
            if let l @ Some(_) = &root.as_ref().unwrap().borrow().left {
                queue.push_back((l.clone(), ret + 1));
            }
            if let r @ Some(_) = &root.as_ref().unwrap().borrow().right {
                queue.push_back((r.clone(), ret + 1));
            }
        }

        ret
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;
    use crate::util::tree::to_tree;

    #[test]
    fn test_111() {
        assert_eq!(Solution::min_depth(tree![3, 9, 20, null, null, 15, 7]), 2);
        assert_eq!(Solution::min_depth(tree![3, 9, 20, 15, 7]), 2);
        assert_eq!(Solution::min_depth(tree![1, 2]), 2);
    }
}
