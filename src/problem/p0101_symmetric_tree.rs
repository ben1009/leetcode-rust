/// [101] Symmetric Tree
///
/// Given the root of a binary tree, check whether it is a mirror of itself (i.e., symmetric around
/// its center).  
/// <strong class="example">Example 1:
/// <img alt="" src="https://assets.leetcode.com/uploads/2021/02/19/symtree1.jpg" style="width: 354px; height: 291px;" />
/// Input: root = [1,2,2,3,4,4,3]
/// Output: true
///
/// <strong class="example">Example 2:
/// <img alt="" src="https://assets.leetcode.com/uploads/2021/02/19/symtree2.jpg" style="width: 308px; height: 258px;" />
/// Input: root = [1,2,2,null,3,null,3]
/// Output: false
///
///  
/// Constraints:
///
///     The number of nodes in the tree is in the range [1, 1000].
///     -100 <= Node.val <= 100
///
///  
/// Follow up: Could you solve it both recursively and iteratively?
pub struct Solution {}
// problem: https://leetcode.com/problems/symmetric-tree/
// discuss: https://leetcode.com/problems/symmetric-tree/discuss/?currentPage=1&orderBy=most_votes&query=

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
    pub fn is_symmetric(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        if root.is_none() {
            return true;
        }

        Self::is_sym(
            root.as_ref().unwrap().borrow().left.clone(),
            root.as_ref().unwrap().borrow().right.clone(),
        )
    }

    fn is_sym(r: Option<Rc<RefCell<TreeNode>>>, l: Option<Rc<RefCell<TreeNode>>>) -> bool {
        if r.is_none() && l.is_none() {
            return true;
        }
        if r.is_none() || l.is_none() {
            return false;
        }
        if r.as_ref().unwrap().borrow().val != l.as_ref().unwrap().borrow().val {
            return false;
        }

        Self::is_sym(
            r.as_ref().unwrap().borrow().left.clone(),
            l.as_ref().unwrap().borrow().right.clone(),
        ) && Self::is_sym(
            r.as_ref().unwrap().borrow().right.clone(),
            l.as_ref().unwrap().borrow().left.clone(),
        )
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;
    use crate::util::tree::to_tree;

    #[test]
    fn test_101() {
        assert!(Solution::is_symmetric(to_tree(vec![
            Some(1),
            Some(2),
            Some(2),
            Some(3),
            Some(4),
            Some(4),
            Some(3)
        ])));
        assert!(!Solution::is_symmetric(tree![1, 2, 2, null, 3, null, 3]));
    }
}
