use std::cell::RefCell;
use std::rc::Rc;

#[derive(Clone, Debug, PartialEq, Copy)]
enum NodeColor {
    Red,
    Black, 
}
enum Direction{
    Left,
    Right
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

impl Direction{
    pub fn opposite(&self)->Direction{
        match self{
            Direction::Left=>Direction::Right,
            Direction::Right=>Direction::Left
        }
    }
}



impl<T: std::cmp::PartialOrd+std::marker::Copy+ std::fmt::Debug> RedBlackTree<T>{
    pub fn insert(&mut self, value: T){}
    pub fn delete(&mut self, value: T)->Option<T>{
        if self.root.is_none(){
            return None;
        }
        let (deleted, new_root) = self.root.clone().unwrap().borrow_mut().delete(value);
        if new_root.is_some(){
            self.root = new_root.unwrap().clone();
        }
        return deleted;
    }
    /*
    pub fn count_leaves(&self)->u32{}
    pub fn height(&self)->u32{}
    pub fn in_order_traverse(&self){}
    pub fn is_empty(&self)->bool{}
    pub fn print(&self)->String{}*/
    pub fn rotate(parent: &TreeRoot<T>, child: &TreeRoot<T>, node_direction: &Direction){
        let p = parent.clone().unwrap();
        let mut p = p.borrow_mut();
        let c = child.clone().unwrap();
        let mut c = c.borrow_mut();

        let grad = p.parent.clone();
        p.parent = child.clone();
        c.parent = grad;
        match node_direction{
            Direction::Left=>{
                //rotate right
                p.left = c.right.clone();
                c.right = parent.clone();

            },
            Direction::Right=>{
                //rotate left
                p.right = c.left.clone();
                c.left = parent.clone();
            },
        }
    }

}

impl<T: std::cmp::PartialOrd+std::marker::Copy+ std::fmt::Debug> TreeNode <T>{
//impl<T: std::cmp::PartialOrd+std::marker::Copy> TreeNode <T>{
    pub fn insert(&mut self, value: T){}
    pub fn delete(&mut self, value: T)->(Option<T>, Option<TreeRoot<T>>){
        self.set_node_delete();
        //None
       (Some(self.value),None) 
    }

    fn get_min(&self)->T{
        match &self.left{
            None=>self.value,
            Some(nd)=>{
                nd.borrow().get_min()
            }
        }
    }


    fn set_node_delete(&mut self){
        self.parent = None;
        self.left = None;
        self.right = None;
    }

    fn get_direction_to_parent(&self)->Direction{
        assert!(self.parent.is_some());
        let p = self.parent.clone().unwrap();
        let parent = p.borrow();
        assert!(parent.value != self.value);
        if parent.value > self.value{
            Direction::Left
        }
        else{
            Direction::Right
        }
    }

    fn get_sibling(&self)->TreeRoot<T>{
        if self.parent.is_none(){
            return None;
        }
        let p = self.parent.clone().unwrap();
        let direc = self.get_direction_to_parent();
        match direc{
            Direction::Left=>p.borrow().right.clone(),
            Direction::Right=>p.borrow().left.clone(),
        }
    }

}

fn main() {
    let nd:TreeNode<i32> = TreeNode{color: NodeColor::Black,
    value: 5, parent: None, left: None, right: None};
    let left: TreeNode<i32> = TreeNode{color: NodeColor::Black,
    value:2, parent: Some(Rc::new(RefCell::new(nd))), left: None, right:None};
    let right: TreeNode<i32> = TreeNode{color: NodeColor::Black,
    value:12, parent: left.parent.clone(), left: None, right:None};
    
    let mut tree: RedBlackTree<i32> = RedBlackTree{root: left.parent.clone()};
    let _nd = tree.root.clone().unwrap();
    _nd.borrow_mut().left = Some(Rc::new(RefCell::new(left)));
    _nd.borrow_mut().right = Some(Rc::new(RefCell::new(right)));

    println!("{:?}",_nd.borrow_mut().value);
    let d = tree.delete(5);
    println!("{:?} {:?}",tree.root.unwrap().borrow_mut().value, d);
}
