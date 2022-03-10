// https://leetcode-cn.com/problems/binary-tree-right-side-view/

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
      right: None
    }
  }
}
use std::rc::Rc;
use std::cell::RefCell;
use std::collections::VecDeque;
impl Solution {
    pub fn right_side_view(mut root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        let mut ret: Vec<i32> = Vec::new();
        let mut qu: VecDeque<(Rc<RefCell<TreeNode>>, i32)> = VecDeque::new();
        if root.is_none() { return ret; }
        let root = root.unwrap();
        let mut lastdepth = 1;
        let mut lastnode = Some(root.clone());
        qu.push_back((root.clone(), 1));
        while !qu.is_empty() {
            let (node, depth) = qu.pop_front().unwrap();
            if depth > lastdepth && lastnode.is_some() {
                ret.push(lastnode.unwrap().borrow().val);
            }
            lastdepth = depth;
            lastnode = Some(node.clone());
            if node.borrow().left.is_some() {
                qu.push_back((node.borrow().left.clone().unwrap(), depth + 1));
            }
            if node.borrow().right.is_some() {
                qu.push_back((node.borrow().right.clone().unwrap(), depth + 1));
            }
        }
        if lastnode.is_some() {
            ret.push(lastnode.unwrap().borrow().val);
        }
        ret
    }
}