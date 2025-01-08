// Given the head of a linked list, return the list after sorting it in ascending order.

// Example 1:

// Input: head = [4,2,1,3]
// Output: [1,2,3,4]
// Example 2:

// Input: head = [-1,5,3,4,0]
// Output: [-1,0,3,4,5]
// Example 3:

// Input: head = []
// Output: []

// Constraints:

// The number of nodes in the list is in the range [0, 5 * 104].
// -105 <= Node.val <= 105

// Follow up: Can you sort the linked list in O(n logn) time and O(1) memory (i.e. constant space)?

use crate::util::linked_list::ListNode;

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
pub struct Solution;

impl Solution {
    pub fn sort_list_with_vec(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        if head.is_none() || head.as_ref().unwrap().next.is_none() {
            return head;
        }

        let mut t = vec![];
        let mut head = head;
        while let Some(mut n) = head {
            head = n.next.take();
            t.push(n);
        }

        t.sort();

        let mut head = None;
        while let Some(mut item) = t.pop() {
            item.next = head;
            head = Some(item);
        }

        head
    }

    pub fn sort_list(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        if head.is_none() || head.as_ref().unwrap().next.is_none() {
            return head;
        }

        let (h1, h2) = Solution::half_cut(head);
        let h1 = Solution::sort_list(h1);
        let h2 = Solution::sort_list(h2);

        Solution::merge(h1, h2)
    }

    fn merge(
        mut h1: Option<Box<ListNode>>,
        mut h2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        if h1.is_none() {
            return h2;
        }
        if h2.is_none() {
            return h1;
        }

        let mut head_pointer = ListNode::new(0);
        let mut pre = &mut head_pointer;
        while h1.is_some() || h2.is_some() {
            if h1.is_none() {
                pre.next = h2.take();
                break;
            }
            if h2.is_none() {
                pre.next = h1.take();
                break;
            }

            if h1.as_ref().unwrap().val < h2.as_ref().unwrap().val {
                pre.next = Some(Box::new(ListNode::new(h1.as_ref().unwrap().val)));
                // h1.clone();
                h1 = h1.as_mut().unwrap().next.take();
            } else {
                pre.next = Some(Box::new(ListNode::new(h2.as_ref().unwrap().val)));
                // pre.next = h2.clone();
                h2 = h2.as_mut().unwrap().next.take();
            }
            pre = pre.next.as_deref_mut().unwrap();
        }

        head_pointer.next
    }

    fn half_cut(head: Option<Box<ListNode>>) -> (Option<Box<ListNode>>, Option<Box<ListNode>>) {
        if head.is_none() || head.as_ref().unwrap().next.is_none() {
            return (head, None);
        }

        let mut head_pointer = ListNode::new(0);
        head_pointer.next = head.clone();

        let mut slow = &mut head_pointer;
        let mut fast = Some(Box::new(slow.clone()));
        while fast.is_some() && fast.as_ref().unwrap().next.is_some() {
            slow = slow.next.as_mut().unwrap();
            fast = fast.as_mut().unwrap().next.as_mut().unwrap().next.take();
        }
        let head2 = slow.next.take();
        // slow.next = None;

        (head_pointer.next, head2)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::util::linked_list;

    #[test]
    fn test_148() {
        assert_eq!(
            Solution::sort_list(linked_list::to_list(vec![4, 2, 1, 3])),
            linked_list::to_list(vec![1, 2, 3, 4])
        );
        assert_eq!(
            Solution::sort_list(linked_list::to_list(vec![-1, 5, 3, 4, 0])),
            linked_list::to_list(vec![-1, 0, 3, 4, 5])
        );
        assert_eq!(
            Solution::sort_list(linked_list::to_list(vec![])),
            linked_list::to_list(vec![])
        );

        assert_eq!(
            Solution::sort_list_with_vec(linked_list::to_list(vec![4, 2, 1, 3])),
            linked_list::to_list(vec![1, 2, 3, 4])
        );
        assert_eq!(
            Solution::sort_list_with_vec(linked_list::to_list(vec![])),
            linked_list::to_list(vec![])
        );
        assert_eq!(
            Solution::sort_list_with_vec(linked_list::to_list(vec![-1, 5, 3, 4, 0])),
            linked_list::to_list(vec![-1, 0, 3, 4, 5])
        );
    }

    extern crate test;
    use test::{Bencher, black_box};

    #[rustfmt::skip]
    // test problem::p0148_sort_list::tests::bench_sort_list          ... bench:      16,858.58 ns/iter (+/- 419.48)
    // test problem::p0148_sort_list::tests::bench_sort_list_with_vec ... bench:       1,120.72 ns/iter (+/- 54.88)
    #[bench]
    fn bench_sort_list(b: &mut Bencher) {
        b.iter(|| {
            black_box(Solution::sort_list(linked_list::to_list(
                (1..=100).rev().collect(),
            )))
        });
    }

    #[bench]
    fn bench_sort_list_with_vec(b: &mut Bencher) {
        b.iter(|| {
            black_box(Solution::sort_list_with_vec(linked_list::to_list(
                (1..=100).rev().collect(),
            )))
        });
    }
}
