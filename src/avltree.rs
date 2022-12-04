use std::cell::RefCell;
use std::rc::Rc;
use std::fmt::{Debug, Display};

pub use crate::tree::{TreeTrait, TreeNodeTrait, Direction, SimpleTreeTrait};


struct TreeNode<T: Ord+Copy+Debug+Display>{
    pub value: T,
    pub parent: TreeRoot<T>,
    left: TreeRoot<T>, 
    right: TreeRoot<T>
}
type TreeRoot<T> = Option<Rc<RefCell<TreeNode<T>>>>;

pub struct AVLTree<T: Ord+Copy+Debug+Display> {
    root: TreeRoot<T>
}

impl <T: Ord+Copy+Debug+Display> AVLTree<T>{
    pub fn new()->Self{
        AVLTree{root: None}
    }

}
impl<T: Ord+Copy+Debug+Display> SimpleTreeTrait<T> for AVLTree<T>{}
