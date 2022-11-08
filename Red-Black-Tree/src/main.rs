use std::cell::RefCell;
use std::rc::Rc;
#[derive(Clone, Debug, PartialEq)]
enum NodeColor {
    Red,
    Black, 
}
type Tree = Rc<RefCell<TreeNode<u32>>>; 
type RedBlackTree= Option<Tree>;
struct TreeNode<T> {
    pub color: NodeColor,
    pub key: T,
    pub parent: RedBlackTree, 
    left: RedBlackTree, 
    right: RedBlackTree,
}

impl<T> RedBlackTree{
    pub fn insert(&mut self, value: T){}
    pub fn delete(&mut self, value: T){}
    pub fn count_leaves(&self)->u32{}
    pub fn height(&self)->u32{}
    pub fn in_order_traverse(&self){}
    pub fn is_empty(&self)->bool{}
    pub fn print(&self)->String{}
}

fn main() {
    println!("Hello, world!");
}
