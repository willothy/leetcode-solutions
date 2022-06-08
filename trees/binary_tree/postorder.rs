// Definition for a binary tree node.
#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        TreeNode {
            val,
            left: None,
            right: None
        }
    }
}

use std::rc::Rc;
use std::cell::RefCell;
impl Solution {
    pub fn postorder_traversal(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        std::mem::take(Self::traverse(&root, &mut Vec::new()))
    }

    fn traverse<'a>(node: &Option<Rc<RefCell<TreeNode>>>, res: &'a mut Vec<i32>) -> &'a mut Vec<i32> {
        if let Some(n) = node {
            let node = n.borrow();
            Self::traverse(&node.left, res);
            Self::traverse(&node.right, res);
            res.push(node.val);
            return res
        }
        return res
    }
}