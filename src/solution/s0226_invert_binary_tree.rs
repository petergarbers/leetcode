pub struct Solution {}
use crate::util::tree::{to_tree, TreeNode};

use std::cell::RefCell;
use std::rc::Rc;

impl Solution {
    pub fn invert_tree(root: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
        root.map(|node| {
            Solution::invert_tree(node.borrow().right.clone());
            Solution::invert_tree(node.borrow().left.clone());
            let left = node.borrow().left.clone();
            let right = node.borrow().right.clone();
            node.borrow_mut().left = right;
            node.borrow_mut().right = left;
            node
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[macro_use]
    use crate::tree;

    #[test]
    fn test_226() {
        assert_eq!(
            Solution::invert_tree(tree![4, 2, 7, 1, 3, 6, 9]),
            tree![4, 7, 2, 9, 6, 3, 1]
        );
    }
}
