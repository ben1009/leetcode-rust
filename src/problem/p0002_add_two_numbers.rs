/// [2] Add Two Numbers
///
/// You are given two non-empty linked lists representing two non-negative integers. The digits are
/// stored in reverse order, and each of their nodes contains a single digit. Add the two numbers
/// and return the sum as a linked list. You may assume the two numbers do not contain any leading
/// zero, except the number 0 itself.  
/// <strong class="example">Example 1:
/// <img alt="" src="https://assets.leetcode.com/uploads/2020/10/02/addtwonumber1.jpg" style="width: 483px; height: 342px;" />
/// Input: l1 = [2,4,3], l2 = [5,6,4]
/// Output: [7,0,8]
/// Explanation: 342 + 465 = 807.
///
/// <strong class="example">Example 2:
///
/// Input: l1 = [0], l2 = [0]
/// Output: [0]
///
/// <strong class="example">Example 3:
///
/// Input: l1 = [9,9,9,9,9,9,9], l2 = [9,9,9,9]
/// Output: [8,9,9,9,0,0,0,1]
///
///  
/// Constraints:
///
///     The number of nodes in each linked list is in the range [1, 100].
///     0 <= Node.val <= 9
///     It is guaranteed that the list represents a number that does not have leading zeros.
pub struct Solution {}

use crate::util::linked_list::ListNode;

// problem: https://leetcode.com/problems/add-two-numbers/
// discuss: https://leetcode.com/problems/add-two-numbers/discuss/?currentPage=1&orderBy=most_votes&query=

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

impl Solution {
    pub fn add_two_numbers(
        l1: Option<Box<ListNode>>,
        l2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        if l1.is_none() {
            return l2;
        }
        if l2.is_none() {
            return l1;
        }

        let mut root_pointer = ListNode::new(0);
        let mut current = &mut root_pointer;
        let mut carry = 0;
        let mut l1 = l1;
        let mut l2 = l2;
        while l1.is_some() || l2.is_some() || carry != 0 {
            let mut v1 = 0;
            let mut v2 = 0;
            if let Some(mut n1) = l1 {
                v1 = n1.val;
                l1 = n1.next.take();
            }
            if let Some(mut n2) = l2 {
                v2 = n2.val;
                l2 = n2.next.take();
            }
            let v = (v1 + v2 + carry) % 10;
            carry = (v1 + v2 + carry) / 10;
            current.next = Some(Box::new(ListNode::new(v)));
            current = current.next.as_mut().unwrap();
        }

        root_pointer.next
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;
    use crate::util::linked_list::to_list;

    #[test]
    fn test_2() {
        assert_eq!(
            Solution::add_two_numbers(to_list(vec![2, 4, 3]), to_list(vec![5, 6, 4])),
            to_list(vec![7, 0, 8])
        );

        assert_eq!(
            Solution::add_two_numbers(to_list(vec![9, 9, 9, 9]), to_list(vec![9, 9, 9, 9, 9, 9])),
            to_list(vec![8, 9, 9, 9, 0, 0, 1])
        );

        assert_eq!(
            Solution::add_two_numbers(to_list(vec![0]), to_list(vec![0])),
            to_list(vec![0])
        )
    }
}
