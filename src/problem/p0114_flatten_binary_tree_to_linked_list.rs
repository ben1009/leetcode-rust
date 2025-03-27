/// [114] Flatten Binary Tree to Linked List
///
/// Given the root of a binary tree, flatten the tree into a "linked list":
///
///     The "linked list" should use the same TreeNode class where the right child pointer points to
/// the next node in the list and the left child pointer is always null.     The "linked list" should be in the same order as a <a href="https://en.wikipedia.org/wiki/Tree_traversal#Pre-order,_NLR" target="_blank">pre-order traversal</a> of the binary tree.
///
///  
/// <strong class="example">Example 1:
/// <img alt="" src="https://assets.leetcode.com/uploads/2021/01/14/flaten.jpg" style="width: 500px; height: 226px;" />
/// Input: root = [1,2,5,3,4,null,6]
/// Output: [1,null,2,null,3,null,4,null,5,null,6]
///
/// <strong class="example">Example 2:
///
/// Input: root = []
/// Output: []
///
/// <strong class="example">Example 3:
///
/// Input: root = [0]
/// Output: [0]
///
///  
/// Constraints:
///
///     The number of nodes in the tree is in the range [0, 2000].
///     -100 <= Node.val <= 100
///
///  
/// Follow up: Can you flatten the tree in-place (with O(1) extra space)?
pub struct Solution {}
// problem: https://leetcode.com/problems/flatten-binary-tree-to-linked-list/
// discuss: https://leetcode.com/problems/flatten-binary-tree-to-linked-list/discuss/?currentPage=1&orderBy=most_votes&query=

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
    #[allow(clippy::assigning_clones)]
    pub fn flatten(root: &Option<Rc<RefCell<TreeNode>>>) {
        if root.is_none() {
            return;
        }

        let mut root = root.as_ref().unwrap().borrow_mut();
        Self::flatten(&root.left.clone());
        Self::flatten(&root.right.clone());

        if root.left.is_none() {
            return;
        }
        let n = Self::find_right_last(root.left.clone());
        n.as_ref()
            .unwrap()
            .borrow_mut()
            .right
            .clone_from(&root.right);
        // n.as_ref().unwrap().borrow_mut().right = root.right.clone();
        root.right = root.left.clone();
        root.left = None
    }

    fn find_right_last(mut root: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
        loop {
            let n = root.as_ref().unwrap().borrow_mut().right.clone();
            match n {
                n @ Some(_) => root = n,
                None => break,
            }
        }

        root
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;
    use crate::util::tree::to_tree;

    #[test]
    fn test_114() {
        let tree_node = tree!(1, 2, 5, 3, 4, null, 6);
        Solution::flatten(&tree_node);
        assert_eq!(
            tree_node,
            tree![1, null, 2, null, 3, null, 4, null, 5, null, 6]
        );
    }
}
