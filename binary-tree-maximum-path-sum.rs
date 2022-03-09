// https://leetcode-cn.com/problems/binary-tree-maximum-path-sum/

struct Solution;
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
            right: None,
        }
    }
}
use std::cell::RefCell;
use std::rc::Rc;
struct Solve {
    max_val: i32,
}
impl Solve {
    pub fn new() -> Self {
        Solve { max_val: std::i32::MIN }
    }
    pub fn maxGain(&mut self, root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        if root.is_none() { return 0; }
        let root = root.unwrap();
        let mut lmax = self.maxGain(root.borrow().left.clone());
        let mut rmax = self.maxGain(root.borrow().right.clone());
        lmax = std::cmp::max(lmax, 0);
        rmax = std::cmp::max(rmax, 0);
        let ret = lmax + rmax + root.borrow().val;
        if ret > self.max_val { self.max_val = ret; }
        return root.borrow().val + std::cmp::max(lmax, rmax);
    }
    pub fn max_path_sum(&mut self, root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        self.maxGain(root);
        self.max_val
    }
}
impl Solution {
    pub fn max_path_sum(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let mut s = Solve::new();
        s.max_path_sum(root)
    }
}