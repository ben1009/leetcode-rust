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

// https://leetcode.com/problems/sort-list/

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
    // need randomize pivot, optimize for == povit, three may quick sort,
    // e.g. [1,1,1,,1,1,4,4,4,6,5,7,8,9], bla, bla
    pub fn sort_list_with_quick_sort(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        if head.is_none() || head.as_ref().unwrap().next.is_none() {
            return head;
        }

        let povit = head.as_ref().unwrap().val;
        let mut less = None;
        let mut great = None;
        let mut current = head;
        while let Some(mut n) = current {
            current = n.next.take();
            if n.val < povit {
                n.next = less;
                less = Some(n);
            } else {
                n.next = great;
                great = Some(n);
            }
        }

        let less = Solution::sort_list_with_quick_sort(less);
        let great = Solution::sort_list_with_quick_sort(great);

        let mut less_head_pointer = ListNode::new(0);
        less_head_pointer.next = less;
        let mut less_tail = &mut less_head_pointer;
        while let Some(ref mut n) = less_tail.next {
            less_tail = n;
        }

        less_tail.next = great;

        less_head_pointer.next
    }

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

    pub fn sort_list_half_cut(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        if head.is_none() || head.as_ref().unwrap().next.is_none() {
            return head;
        }

        let (h1, h2) = Solution::half_cut(head);
        let h1 = Solution::sort_list_half_cut(h1);
        let h2 = Solution::sort_list_half_cut(h2);

        Solution::merge(h1, h2)
    }

    pub fn sort_list_half_cut_len(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        if head.is_none() || head.as_ref().unwrap().next.is_none() {
            return head;
        }

        let (h1, h2) = Solution::half_cut_len(head);
        let h1 = Solution::sort_list_half_cut_len(h1);
        let h2 = Solution::sort_list_half_cut_len(h2);

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
                let n = h1.as_mut().unwrap().next.take();
                pre.next = h1.take();
                h1 = n;
            } else {
                let n = h2.as_mut().unwrap().next.take();
                pre.next = h2.take();
                h2 = n;
            }
            pre = pre.next.as_deref_mut().unwrap();
        }

        head_pointer.next
    }

    fn half_cut_len(
        mut head: Option<Box<ListNode>>,
    ) -> (Option<Box<ListNode>>, Option<Box<ListNode>>) {
        if head.is_none() || head.as_ref().unwrap().next.is_none() {
            return (head, None);
        }

        let len = Solution::len(&mut head);
        let mid = len / 2;

        let mut head_pointer = ListNode::new(0);
        head_pointer.next = head;
        let mut head2 = &mut head_pointer;
        for _i in 0..mid {
            head2 = head2.next.as_mut().unwrap();
        }
        let head2 = head2.next.take();

        (head_pointer.next, head2)
    }

    fn len(head: &mut Option<Box<ListNode>>) -> i32 {
        let mut len = 0;
        let mut head = head;
        while let Some(ref mut n) = head {
            head = &mut n.next;
            len += 1;
        }

        len
    }

    fn half_cut(head: Option<Box<ListNode>>) -> (Option<Box<ListNode>>, Option<Box<ListNode>>) {
        if head.is_none() || head.as_ref().unwrap().next.is_none() {
            return (head, None);
        }

        let mut head = head;
        let mut head_pointer = ListNode::new(0);
        head_pointer.next = head.take();

        let mut slow = &mut head_pointer;
        let mut fast = Some(Box::new(slow.clone())); // clone() kind of slow down the algorithm
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
        // sort_list_with_quick_sort
        assert_eq!(
            Solution::sort_list_with_quick_sort(linked_list::to_list(vec![4, 2, 1, 3])),
            linked_list::to_list(vec![1, 2, 3, 4])
        );
        assert_eq!(
            Solution::sort_list_with_quick_sort(linked_list::to_list(vec![-1, 5, 3, 4, 0])),
            linked_list::to_list(vec![-1, 0, 3, 4, 5])
        );
        assert_eq!(
            Solution::sort_list_with_quick_sort(linked_list::to_list(vec![])),
            linked_list::to_list(vec![])
        );

        // sort_list_half_cut
        assert_eq!(
            Solution::sort_list_half_cut(linked_list::to_list(vec![4, 2, 1, 3])),
            linked_list::to_list(vec![1, 2, 3, 4])
        );
        assert_eq!(
            Solution::sort_list_half_cut(linked_list::to_list(vec![-1, 5, 3, 4, 0])),
            linked_list::to_list(vec![-1, 0, 3, 4, 5])
        );
        assert_eq!(
            Solution::sort_list_half_cut(linked_list::to_list(vec![])),
            linked_list::to_list(vec![])
        );

        // sort_list_half_cut_len
        assert_eq!(
            Solution::sort_list_half_cut_len(linked_list::to_list(vec![4, 2, 1, 3])),
            linked_list::to_list(vec![1, 2, 3, 4])
        );
        assert_eq!(
            Solution::sort_list_half_cut_len(linked_list::to_list(vec![-1, 5, 3, 4, 0])),
            linked_list::to_list(vec![-1, 0, 3, 4, 5])
        );
        assert_eq!(
            Solution::sort_list_half_cut_len(linked_list::to_list(vec![])),
            linked_list::to_list(vec![])
        );

        // sort_list_with_vec
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
    // test problem::p0148_sort_list::tests::bench_sort_list_half_cut     ... bench:       8,389.08 ns/iter (+/- 243.06)
    // test problem::p0148_sort_list::tests::bench_sort_list_half_cut_len ... bench:       1,913.11 ns/iter (+/- 57.90)
    // test problem::p0148_sort_list::tests::bench_sort_list_with_quick   ... bench:      16,013.85 ns/iter (+/- 116.08)
    // test problem::p0148_sort_list::tests::bench_sort_list_with_vec     ... bench:       1,142.24 ns/iter (+/- 35.95)
    
    #[bench]
    fn bench_sort_list_with_quick(b: &mut Bencher) {
        b.iter(|| {
            black_box(Solution::sort_list_with_quick_sort(linked_list::to_list(
                (1..=100).rev().collect(),
            )))
        });
    }

    #[bench]
    fn bench_sort_list_half_cut(b: &mut Bencher) {
        b.iter(|| {
            black_box(Solution::sort_list_half_cut(linked_list::to_list(
                (1..=100).rev().collect(),
            )))
        });
    }

    #[bench]
    fn bench_sort_list_half_cut_len(b: &mut Bencher) {
        b.iter(|| {
            black_box(Solution::sort_list_half_cut_len(linked_list::to_list(
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
