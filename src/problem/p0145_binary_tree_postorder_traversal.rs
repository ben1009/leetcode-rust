/// [145] Binary Tree Postorder Traversal
///
/// Given the root of a binary tree, return the postorder traversal of its nodes' values.
///  
/// <strong class="example">Example 1:
/// <img alt="" src="https://assets.leetcode.com/uploads/2020/08/28/pre1.jpg" style="width: 127px; height: 200px;" />
/// Input: root = [1,null,2,3]
/// Output: [3,2,1]
///
/// <strong class="example">Example 2:
///
/// Input: root = []
/// Output: []
///
/// <strong class="example">Example 3:
///
/// Input: root = [1]
/// Output: [1]
///
///  
/// Constraints:
///
///     The number of the nodes in the tree is in the range [0, 100].
///     -100 <= Node.val <= 100
///
///  
/// Follow up: Recursive solution is trivial, could you do it iteratively?
pub struct Solution {}
// problem: https://leetcode.com/problems/binary-tree-postorder-traversal/
// discuss: https://leetcode.com/problems/binary-tree-postorder-traversal/discuss/?currentPage=1&orderBy=most_votes&query=

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
    pub fn postorder_traversal(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        if root.is_none() {
            return vec![];
        }

        let mut root = root;
        let mut stack = vec![];
        let mut ret = vec![];
        let mut pre = Some(Rc::new(RefCell::new(TreeNode::new(0))));
        while !stack.is_empty() || root.is_some() {
            if let Some(n) = root {
                stack.push(n.clone());
                root = n.borrow_mut().left.take();
            } else {
                let n = stack[stack.len() - 1].clone();
                if n.borrow().right.is_none() || n.borrow().right == pre {
                    stack.pop();
                    ret.push(n.borrow().val);
                    pre = Some(n);
                } else {
                    root = n.borrow_mut().right.take();
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
    use crate::util::tree::to_tree;

    #[test]
    fn test_145() {
        assert_eq!(
            Solution::postorder_traversal(tree![1, null, 2, 3]),
            vec![3, 2, 1]
        );
        assert_eq!(Solution::postorder_traversal(tree![1]), vec![1]);
        assert_eq!(Solution::postorder_traversal(tree![1, 2]), vec![2, 1]);
    }
}
