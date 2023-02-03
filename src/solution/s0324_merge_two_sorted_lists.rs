// // Definition for singly-linked list.
// #[derive(PartialEq, Eq, Clone, Debug)]
// pub struct ListNode {
//   pub val: i32,
//   pub next: Option<Box<ListNode>>
// }

// impl ListNode {
//   #[inline]
//   fn new(val: i32) -> Self {
//     ListNode {
//       next: None,
//       val
//     }
//   }
// }

use crate::util::linked_list::{to_list, ListNode};

pub struct Solution {}

impl Solution {
    pub fn merge_two_lists(list1: Option<Box<ListNode>>, list2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        match (list1, list2) {
            (None, None) => None,
            (None, Some(l2)) => Some(l2),
            (Some(l1), None) => Some(l1),
            (Some(l1), Some(l2)) => match l1.val <= l2.val {
                true => Some(Box::new(ListNode { 
                    val: l1.val,
                    next: Self::merge_two_lists(l1.next, Some(l2))
                })),
                false => Some(Box::new(ListNode {
                    val: l2.val,
                    next: Self::merge_two_lists(Some(l1), l2.next)
                })),
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::util::linked_list::{to_list, ListNode};

    #[test]
    fn moo() {
        let result = 2 + 2;
        println!("{:?}", to_list(vec![1, 2, 3]));
        let l1 = to_list(vec![1, 2, 3]);
        let l2 = to_list(vec![1, 3, 4]);
        let result = to_list(vec![1, 1, 2, 3, 3, 4]);
        assert_eq!(Solution::merge_two_lists(l1, l2), result);
    }
}
