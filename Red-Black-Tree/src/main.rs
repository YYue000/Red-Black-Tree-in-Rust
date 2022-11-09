use std::cell::RefCell;
use std::rc::Rc;
#[derive(Clone, Debug, PartialEq)]
enum NodeColor {
    Red,
    Black, 
}

struct TreeNode<T> {
    pub color: NodeColor,
    pub key: T,
    pub parent: RedBlackTree<T>, 
    left: RedBlackTree<T>, 
    right: RedBlackTree<T>
}
struct RedBlackTree<T>{
    root: Option<Rc<RefCell<TreeNode<T>>>>,
}

impl<T> RedBlackTree<T>{
    pub fn insert(&mut self, value: T){}
    pub fn delete(&mut self, value: T)->Option<T>{

    }
    pub fn count_leaves(&self)->u32{}
    pub fn height(&self)->u32{}
    pub fn in_order_traverse(&self){}
    pub fn is_empty(&self)->bool{}
    pub fn print(&self)->String{}
}

impl<T: std::cmp::PartialOrd> TreeNode <T>{
    pub fn insert(&mut self, value: T){}
    pub fn delete(&mut self, value: T)->Option<T>{
        match value{
            v if v < self.key=>{
                match self.left.root{
                    None=>{return None;}
                    Some(t)=>{return t.borrow_mut().delete(value);}
                }
            },
            v if v > self.key=>{
                match self.right.root{
                    None=>{return None;}
                    Some(t)=>{return t.borrow_mut().delete(value);}
                }
            },
            _=>{}
        };

        None
    }
}
fn main() {
    println!("Hello, world!");
}
