//use std::borrow::Borrow;
//use std::borrow::{BorrowMut};
use std::cell::RefCell;
use std::rc::Rc;
use std::cmp::PartialOrd;
use std::marker::Copy;
#[derive(Clone, Debug, PartialEq)]
enum NodeColor {
    Red,
    Black, 
}

type TreeRoot<T> = Option<Rc<RefCell<TreeNode<T>>>>;
struct TreeNode<T> {
    pub color: NodeColor,
    pub value: T,
    pub parent: TreeRoot<T>, 
    left: TreeRoot<T>, 
    right: TreeRoot<T>
}
struct RedBlackTree<T>{
    root: TreeRoot<T>
}

impl<T:PartialOrd + Copy> RedBlackTree<T>{
    pub fn new() -> Self {
        RedBlackTree { root: None }
    }
    pub fn insert(&mut self, value: T){
        let root=self.root;
        root=match root {
            /*Some(root_rc) => {
                let root_node = root_rc.borrow();
                TreeNode::insert(root_node.borrow_mut(),value);
            }*/
            Some(root) => {
                //let root_node=root.borrow();
                TreeNode::insert(root,value)
                //Some(Rc::new(RefCell::new(new_root)))
            }
            None => todo!(),
        }
        
    }
    /*pub fn delete(&mut self, value: T)->Option<T>{}
    pub fn count_leaves(&self)->u32{}
    pub fn height(&self)->u32{}
    pub fn in_order_traverse(&self){}
    pub fn is_empty(&self)->bool{}
    pub fn print(&self)->String{}*/

}



impl<T: PartialOrd+ Copy> TreeNode <T>{
    fn new(value: T) -> Self {
        TreeNode {
            color: NodeColor::Red,
            value: value,
            parent: None,
            left: None,
            right: None,
        }
    }


    fn new_with_parent(value: T, parent: Rc<RefCell<TreeNode<T>>>) -> Self {
        TreeNode {
            color: NodeColor::Red,
            value: value,
            parent: Some(parent),
            left: None,
            right: None,
        }
    }
    pub fn insert(node:Rc<RefCell<TreeNode<T>>>, value: T) -> TreeRoot<T>{
        if node.borrow().value ==value{
            //return Some(Rc::new(RefCell::new(*self)));
            return Some(node);
        }else if node.borrow().value >value{
            let left=node.borrow().left.clone();
            match left {
                Some(left_node) => {
                    //let left_node = left_rc.borrow();
                    Self::insert(left_node,value);
                }
                None => {
                    let left=Self::new_with_parent(value, node);
                    node.borrow_mut().left= Some(Rc::new(RefCell::new(left)));
                    left.insert_recolor();
                },
            }

        }else {
            let right=node.borrow().right.clone();
            match right {
                Some(right_node) => {
                    //let right_node = right_rc.borrow();
                    Self::insert(right_node,value);
                }
                None => {
                    let right=TreeNode::new_with_parent(value, node);
                    node.borrow_mut().right = Some(Rc::new(RefCell::new(right)));
                    right.insert_recolor();
                },
            }
        }
        //return Some(Rc::new(RefCell::new(self.get_root())));
        return Self::get_root(node);
    }
    pub fn insert_recolor(&mut self){
        match self.parent {
            Some(parent) =>{
                //1.if parent is black, no need to change
                if(parent.borrow().color==NodeColor::Black){
                    ();
                }
                //2.if parent is red
                else{
                    let grand_parent=parent.borrow().parent.clone();
                    match grand_parent {
                        Some(grand_parent) => {
                            if(grand_parent.borrow().color==NodeColor::Red){
                                panic!("insert into tree, red parent node have red grand parent.");
                            }
                            
                            
                        }
                        None => {

                        }
                    }
                }
                //current node is the root and 

            },
            None => {

            },

        }

    }
    pub fn get_root(node:Rc<RefCell<TreeNode<T>>>)-> TreeRoot<T>{
        let parent=node.borrow().parent.clone();
        match parent {
            Some(p) => {
                Self::get_root(p)
            },
            None => Some(node),                                                                        lf,
        }

    }
    pub fn set_black(&mut self){
        todo!("set node color to black")
    }
    pub fn is_left(&self)->bool{
        todo!();
    }

    pub fn delete(&mut self, value: T)->Option<T>{
    }

}
fn main() {
    println!("Hello, world!");
}
