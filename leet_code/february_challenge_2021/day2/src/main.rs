fn main() {
    println!("Hello, world!");
}

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
            right: None,
        }
    }
}

struct Solution;

use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
    pub fn trim_bst(
        root: Option<Rc<RefCell<TreeNode>>>,
        low: i32,
        high: i32,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        if root.is_none() {
            return None;
        }

        let root = root.unwrap();
        let borrowed_root = root.borrow();

        if borrowed_root.val < low {
            return Solution::trim_bst(borrowed_root.right.clone(), low, high);
        }

        if borrowed_root.val > high {
            return Solution::trim_bst(borrowed_root.left.clone(), low, high);
        }

        Some(Rc::new(RefCell::new(TreeNode {
            val: borrowed_root.val,
            left: Solution::trim_bst(borrowed_root.left.clone(), low, high),
            right: Solution::trim_bst(borrowed_root.right.clone(), low, high),
        })))
    }
}
