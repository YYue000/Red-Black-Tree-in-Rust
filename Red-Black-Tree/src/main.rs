use std::cell::RefCell;
use std::rc::Rc;
#[derive(Clone, Debug, PartialEq)] enum NodeColor {
    Red,
    Black, 
}
type Tree = Rc<RefCell<TreeNode<u32>>>; 
type RedBlackTree= Option<Tree>;
struct TreeNode<T> {
    pub color: NodeColor,
    pub key: T,
    pub parent: RedBlackTree, left: RedBlackTree, right: RedBlackTree,
}

impl RedBlackTree{
    fn insert(){}
    fn delete(){}
    fn count_leaves(){}
    fn height(){}
    fn in_order_traverse(){}
    fn is_empty(){}
    fn print(){}
}

fn main() {
    println!("Hello, world!");
}
