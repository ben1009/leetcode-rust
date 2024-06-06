// Given the root of a binary tree, return the preorder traversal of its nodes' values.

// Example 1:

// Input: root = [1,null,2,3]
// Output: [1,2,3]
// Example 2:

// Input: root = []
// Output: []
// Example 3:

// Input: root = [1]
// Output: [1]

// Constraints:

// The number of nodes in the tree is in the range [0, 100].
// -100 <= Node.val <= 100

// Follow up: Recursive solution is trivial, could you do it iteratively?
pub struct Solution {}
// https://leetcode.com/problems/binary-tree-preorder-traversal/description/

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
    pub fn preorder_traversal(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        if root.is_none() {
            return vec![];
        }

        let mut stack = vec![];
        let mut root = root;
        let mut ret = vec![];
        while root.is_some() || !stack.is_empty() {
            if root.is_some() {
                stack.push(root.clone());
                ret.push(root.as_ref().unwrap().borrow().val);
                let t = root.as_ref().unwrap().borrow().left.clone();
                root = t;
                continue;
            }
            root = stack.pop().unwrap();
            let t = root.as_ref().unwrap().borrow().right.clone();
            root = t;
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
    fn test_144() {
        assert_eq!(
            Solution::preorder_traversal(tree![1, null, 2, 3]),
            vec![1, 2, 3]
        );
        assert_eq!(Solution::preorder_traversal(tree![1]), vec![1]);
        assert_eq!(Solution::preorder_traversal(tree![]), vec![]);
    }
}
