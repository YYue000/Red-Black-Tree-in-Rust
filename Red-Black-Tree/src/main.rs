use std::cell::RefCell;
use std::rc::Rc;
#[derive(Clone, Debug, PartialEq)]
enum NodeColor {
    Red,
    Black, 
}

type TreeRoot<T> = Option<Rc<RefCell<TreeNode<T>>>>;
struct TreeNode<T> {
    pub color: NodeColor,
    pub key: T,
    pub parent: TreeRoot<T>, 
    left: TreeRoot<T>, 
    right: TreeRoot<T>
}
struct RedBlackTree<T>{
    root: TreeRoot<T>
}

/*impl<T> RedBlackTree<T>{
    pub fn insert(&mut self, value: T){}
    pub fn delete(&mut self, value: T)->Option<T>{}
    pub fn count_leaves(&self)->u32{}
    pub fn height(&self)->u32{}
    pub fn in_order_traverse(&self){}
    pub fn is_empty(&self)->bool{}
    pub fn print(&self)->String{}
}*/

// impl<T: std::cmp::PartialOrd+std::marker::Copy> TreeNode <T>{
//     pub fn insert(&mut self, value: T){}
//     pub fn delete(&mut self, value: T)->Option<T>{
//     }

use Red_Black_Tree::Avl_tree::*;

fn main() {
    let mut test = AvlNode {
        value: 4,
        left: None,
        right: None,
    };

    println!("Hello, world!");
}
