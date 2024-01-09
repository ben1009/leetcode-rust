/// [23] Merge k Sorted Lists
///
/// You are given an array of k linked-lists lists, each linked-list is sorted in ascending order.
/// Merge all the linked-lists into one sorted linked-list and return it.
///  
/// <strong class="example">Example 1:
///
/// Input: lists = [[1,4,5],[1,3,4],[2,6]]
/// Output: [1,1,2,3,4,4,5,6]
/// Explanation: The linked-lists are:
/// [
///   1->4->5,
///   1->3->4,
///   2->6
/// ]
/// merging them into one sorted list:
/// 1->1->2->3->4->4->5->6
///
/// <strong class="example">Example 2:
///
/// Input: lists = []
/// Output: []
///
/// <strong class="example">Example 3:
///
/// Input: lists = [[]]
/// Output: []
///
///  
/// Constraints:
///
///     k == lists.length
///     0 <= k <= 10^4
///     0 <= lists[i].length <= 500
///     -10^4 <= lists[i][j] <= 10^4
///     lists[i] is sorted in ascending order.
///     The sum of lists[i].length will not exceed 10^4.
pub struct Solution {}
use std::{cmp::Reverse, collections::BinaryHeap};

use crate::util::linked_list::ListNode;

// problem: https://leetcode.com/problems/merge-k-sorted-lists/
// discuss: https://leetcode.com/problems/merge-k-sorted-lists/discuss/?currentPage=1&orderBy=most_votes&query=

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
    pub fn merge_k_lists(lists: Vec<Option<Box<ListNode>>>) -> Option<Box<ListNode>> {
        if lists.is_empty() {
            return None;
        }

        let mut pq = BinaryHeap::new();
        for l in lists {
            if l.is_some() {
                pq.push(Reverse(l));
            }
        }

        let mut ret = Some(Box::new(ListNode::new(0)));
        let mut head = &mut ret;
        while !pq.is_empty() {
            let node = pq.pop().unwrap().0.unwrap();
            head.as_mut().unwrap().next = Some(Box::new(ListNode::new(node.val)));
            head = &mut head.as_mut().unwrap().next;
            if node.next.is_some() {
                pq.push(Reverse(node.next));
            }
        }

        ret.unwrap().next
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_23() {
        assert_eq!(
            Solution::merge_k_lists(vec![list!(1, 4), list!(2, 3)]),
            Some(Box::new(ListNode {
                val: 1,
                next: Some(Box::new(ListNode {
                    val: 2,
                    next: Some(Box::new(ListNode {
                        val: 3,
                        next: Some(Box::new(ListNode { val: 4, next: None }))
                    }))
                }))
            }))
        );
        assert_eq!(Solution::merge_k_lists(vec![]), None);
        assert_eq!(Solution::merge_k_lists(vec![None]), None);
        assert_eq!(
            Solution::merge_k_lists(vec![list!(1)]),
            Some(Box::new(ListNode { val: 1, next: None }))
        );
    }
}
