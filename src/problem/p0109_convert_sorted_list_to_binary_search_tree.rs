/**
 * [109] Convert Sorted List to Binary Search Tree
 *
 * Given the head of a singly linked list where elements are sorted in ascending order, convert it to a <span data-keyword="height-balanced">height-balanced</span> binary search tree.
 *  
 * <strong class="example">Example 1:
 * <img alt="" src="https://assets.leetcode.com/uploads/2020/08/17/linked.jpg" style="width: 500px; height: 388px;" />
 * Input: head = [-10,-3,0,5,9]
 * Output: [0,-3,9,-10,null,5]
 * Explanation: One possible answer is [0,-3,9,-10,null,5], which represents the shown height balanced BST.
 *
 * <strong class="example">Example 2:
 *
 * Input: head = []
 * Output: []
 *
 *  
 * Constraints:
 *
 *     The number of nodes in head is in the range [0, 2 * 10^4].
 *     -10^5 <= Node.val <= 10^5
 *
 */
pub struct Solution {}
use reqwest::header;

use crate::util::linked_list::{to_list, ListNode};
use crate::util::tree::{to_tree, TreeNode};

// problem: https://leetcode.com/problems/convert-sorted-list-to-binary-search-tree/
// discuss: https://leetcode.com/problems/convert-sorted-list-to-binary-search-tree/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

// Definition for singly-linked list.
// #[derive(PartialEq, Eq, Clone, Debug)]
// pub struct ListNode {
//   pub val: i32,
//   pub next: Option<Box<ListNode>>
// }
//
// impl ListNode {
//   #[inline]
//   fn new(val: i32) -> Self {
//     ListNode {
//       next: None,
//       val
//     }
//   }
// }
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
impl Solution {
    pub fn sorted_list_to_bst(head: Option<Box<ListNode>>) -> Option<Rc<RefCell<TreeNode>>> {
        head.as_ref()?;

        Self::to_bst(head, None)
    }

    fn to_bst(
        head: Option<Box<ListNode>>,
        tail: Option<Box<ListNode>>,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        if head == tail {
            return None;
        }

        // TODO: could push into a vec, get mid by idx, instead of traversing again and again for better time complexity
        let mid = Self::find_mid(head.clone(), tail.clone());
        let mut n = TreeNode::new(mid.as_ref().unwrap().val);
        n.left = Self::to_bst(head, mid.clone());
        n.right = Self::to_bst(mid.unwrap().next, tail);

        Some(Rc::new(RefCell::new(n)))
    }

    fn find_mid(head: Option<Box<ListNode>>, tail: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut slow = head.clone();
        let mut fast = head;
        while fast != tail && fast.as_ref().unwrap().next != tail {
            fast = fast.as_ref().unwrap().next.as_ref().unwrap().next.clone();
            slow = slow.as_ref().unwrap().next.clone();
        }

        slow
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_109() {
        assert_eq!(
            Solution::sorted_list_to_bst(linked![-10, -3, 0, 5, 9]),
            tree![0, -3, 9, -10, null, 5]
        );
        assert_eq!(Solution::sorted_list_to_bst(linked![]), tree![]);
        assert_eq!(Solution::sorted_list_to_bst(linked![-1]), tree![-1]);
    }
}
