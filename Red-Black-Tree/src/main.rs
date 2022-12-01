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
    fn is_left(&self)->bool{
        match self{
            Direction::Left=>true,
            Direction::Right=>false
        }
    }
}



impl<T: std::cmp::PartialOrd+std::marker::Copy+ std::fmt::Debug> RedBlackTree<T>{
    pub fn insert(&mut self, value: T){}
    pub fn delete(&mut self, value: T)->Option<T>{
        let (deleted, new_root) = delete_node(self.root.clone(), value);
        if new_root.is_some(){
            self.root = new_root.unwrap().clone();
        }
        return deleted;
    }
    /*
    pub fn count_leaves(&self)->u32{}
    pub fn height(&self)->u32{}
    pub fn in_order_traverse(&self){}
    pub fn is_empty(&self)->bool{}*/
    fn print(&self) {
        if self.root.is_none() {
            println!("Empty tree!");
            return;
        }
        self.root.clone().unwrap().borrow().print_tree("  ".to_string());
    }

    pub fn rotate(parent: &TreeRoot<T>, child: &TreeRoot<T>, node_direction: &Direction){
    }

}

impl<T: std::cmp::PartialOrd+std::marker::Copy+ std::fmt::Debug> TreeNode <T>{
//impl<T: std::cmp::PartialOrd+std::marker::Copy> TreeNode <T>{

    fn get_min(&self)->T{
        match &self.left{
            None=>self.value,
            Some(nd)=>{
                nd.borrow().get_min()
            }
        }
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

    fn is_leave(&self)->bool{
        self.left.is_none() && self.right.is_none()
    }

    fn print_tree(&self, ident: String) {

        println!(
            "(Color: {:?}, Value: {:?}, Is Leave: {:?})",
            self.color,
            self.value,
            self.is_leave(),
        );

        if self.left.is_some() {
            print!("{}left: ", ident);
            self.left.clone().unwrap().borrow().print_tree(ident.clone() + "  ");
        }

        if self.right.is_some() {
            print!("{}right: ", ident);
            self.right.clone().unwrap().borrow().print_tree(ident.clone() + "  ");
        }
    }


}
fn delete_node<T: std::cmp::PartialOrd+std::marker::Copy+ std::fmt::Debug>(root: TreeRoot<T>, value: T)->(Option<T>, Option<TreeRoot<T>>){
    if root.is_none(){
        return (None, None);
    }

    let node = root.clone().unwrap();


   return (Some(node.borrow().value),None);
    }


fn main() {
}
