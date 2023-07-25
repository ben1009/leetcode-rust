/// [617] Merge Two Binary Trees
///
/// You are given two binary trees root1 and root2.
/// Imagine that when you put one of them to cover the other, some nodes of the two trees are
/// overlapped while the others are not. You need to merge the two trees into a new binary tree. The
/// merge rule is that if two nodes overlap, then sum node values up as the new value of the merged
/// node. Otherwise, the NOT null node will be used as the node of the new tree. Return the merged
/// tree. Note: The merging process must start from the root nodes of both trees.
///  
/// <strong class="example">Example 1:
/// <img alt="" src="https://assets.leetcode.com/uploads/2021/02/05/merge.jpg" style="width: 600px; height: 163px;" />
/// Input: root1 = [1,3,2,5], root2 = [2,1,3,null,4,null,7]
/// Output: [3,4,5,5,4,null,7]
///
/// <strong class="example">Example 2:
///
/// Input: root1 = [1], root2 = [1,2]
/// Output: [2,2]
///
///  
/// Constraints:
///
///     The number of nodes in both trees is in the range [0, 2000].
///     -10^4 <= Node.val <= 10^4
///
pub struct Solution {}
// problem: https://leetcode.com/problems/merge-two-binary-trees/
// discuss: https://leetcode.com/problems/merge-two-binary-trees/discuss/?currentPage=1&orderBy=most_votes&query=

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

use crate::util::tree::TreeNode;
impl Solution {
    pub fn merge_trees(
        root1: Option<Rc<RefCell<TreeNode>>>,
        root2: Option<Rc<RefCell<TreeNode>>>,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        if root1.is_none() {
            return root2;
        }
        if root2.is_none() {
            return root1;
        }

        let val = root1.as_ref().unwrap().borrow().val + root2.as_ref().unwrap().borrow().val;
        let root = Some(Rc::new(RefCell::new(TreeNode::new(val))));
        root.as_ref().unwrap().borrow_mut().left = Self::merge_trees(
            root1.as_ref().unwrap().borrow().left.clone(),
            root2.as_ref().unwrap().borrow().left.clone(),
        );
        root.as_ref().unwrap().borrow_mut().right = Self::merge_trees(
            root1.as_ref().unwrap().borrow().right.clone(),
            root2.as_ref().unwrap().borrow().right.clone(),
        );

        root
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;
    use crate::util::tree::to_tree;

    #[test]
    fn test_617() {
        assert_eq!(
            Solution::merge_trees(tree![1, 3, 2, 5], tree![2, 1, 3, null, 4, null, 7]),
            tree![3, 4, 5, 5, 4, null, 7]
        );
        assert_eq!(Solution::merge_trees(tree![1], tree![1, 2]), tree![2, 2]);
    }
}
