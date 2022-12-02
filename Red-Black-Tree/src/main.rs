use std::cell::RefCell;
use std::clone;
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

impl<T: std::cmp::PartialOrd+std::marker::Copy+ std::fmt::Debug> RedBlackTree<T>{
    pub fn new() -> Self {
        RedBlackTree { root: None }
    }
    pub fn insert(&mut self, value: T){
        
        let root=self.root.clone();
        /*if root.is_some(){
            self.root=insert_node(root, value);
        }else{
            let mut new_root=Rc::new(RefCell::new(TreeNode::new(value)));
            self.root=Some(TreeNode::set_black(new_root))
        }*/
        
        self.root=match root {
            /*Some(root_rc) => {
                let root_node = root_rc.borrow();
                TreeNode::insert(root_node.borrow_mut(),value);
            }*/
            Some(root) => {
                //let root_node=root.borrow();
                insert_node(root,value)
                //Some(Rc::new(RefCell::new(new_root)))
            }
            None => {
                let mut new_root=Rc::new(RefCell::new(TreeNode::new(value)));
                
                Some(TreeNode::set_black(new_root))
            },
        }
        
        
    }
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
    pub fn is_valid_red_black_tree(root: TreeRoot<T>) -> bool {
        let result = TreeNode::calculate_black_height(root);
        match result {
            Some(_) => true,
            None => false,
        }
    }

}



impl<T: std::cmp::PartialOrd+std::marker::Copy+ std::fmt::Debug> TreeNode <T>{
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
    
    

    // Helper function for RBTree::is_valid_red_black_tree
    fn calculate_black_height(node:TreeRoot<T>) -> Option<usize> {
        match node {
            None => Some(1),
            Some(node) => {
                let left_height = Self::calculate_black_height(node.borrow().left.clone());
                let right_height = Self::calculate_black_height(node.borrow().right.clone());
                match (left_height, right_height) {
                    (Some(left_height), Some(right_height)) => {
                        if left_height != right_height {
                            //The 2 children have unequal depths
                            None
                        } else {
                            let node_color = &node.borrow().color;
                            //Return the black depth of children,plus 1 if the node is black
                            match node_color {
                                NodeColor::Red => Some(left_height),
                                NodeColor::Black => Some(left_height + 1),
                            }
                        }
                    }
                    _ => None,
                }
            }
        }
    }
    



    pub fn get_root(node:Rc<RefCell<TreeNode<T>>>)-> TreeRoot<T>{
        let parent=node.borrow().parent.clone();
        match parent {
            Some(p) => {
                Self::get_root(p)
            },
            None => Some(node),                                                                  
        }

    }
    fn set_red(node:Rc<RefCell<TreeNode<T>>>) -> Rc<RefCell<TreeNode<T>>> {
        node.borrow_mut().color = NodeColor::Red;
        return node;
    }

    // Helper function for maintaining
    fn set_black(node:Rc<RefCell<TreeNode<T>>>) -> Rc<RefCell<TreeNode<T>>> {
        node.borrow_mut().color = NodeColor::Black;
        return node;
    }

    pub fn is_left(node:Rc<RefCell<TreeNode<T>>>)->bool{
        match node.borrow().parent.clone() {
            Some(parent) => match parent.borrow().left.clone() {
                Some(left) => Rc::ptr_eq(&left, &node),
                None => false,
            },
            _ => false,
        }
    }
    pub fn is_right(node:Rc<RefCell<TreeNode<T>>>)->bool{
        // Return true if the node is the right child of its parent.
        match node.borrow().parent.clone() {
            Some(parent) => match parent.borrow().right.clone() {
                Some(right) => Rc::ptr_eq(&right, &node),
                None => false,
            },
            _ => false,
        }
    }
    /*pub fn delete(&mut self, value: T)->Option<T>{
    }*/
    pub fn rotate_left(node:Rc<RefCell<TreeNode<T>>>){
        let parent = node.borrow().parent.clone();
        let right = node.borrow().right.clone();

        node.borrow_mut().right = right.clone().unwrap().borrow().left.clone();
        if node.borrow().right.is_some() {
            let right = node.borrow().right.clone().unwrap();
            right.borrow_mut().parent = Some(node.clone());
        }
        node.borrow_mut().parent = right.clone();
        right.clone().unwrap().borrow_mut().left = Some(node.clone());
        if parent.is_some() {
            let left = parent.clone().unwrap().borrow().left.clone();
            match left {
                Some(left) if Rc::ptr_eq(&left, &node) => {
                    parent.clone().unwrap().borrow_mut().left = right.clone();
                }
                _ => parent.clone().unwrap().borrow_mut().right = right.clone(),
            }
        }

        right.clone().unwrap().borrow_mut().parent = parent;
    }
    pub fn rotate_right(node:Rc<RefCell<TreeNode<T>>>){
        let parent = node.borrow().parent.clone();
        let left = node.borrow().left.clone();

        node.borrow_mut().left = left.clone().unwrap().borrow().right.clone();
        if node.borrow().left.is_some() {
            let left = node.borrow().left.clone().unwrap();
            left.borrow_mut().parent = Some(node.clone());
        }
        node.borrow_mut().parent = left.clone();
        left.clone().unwrap().borrow_mut().right = Some(node.clone());
        if parent.is_some() {
            let right = parent.clone().unwrap().borrow().right.clone();
            match right {
                Some(right) if Rc::ptr_eq(&right, &node) => {
                    parent.clone().unwrap().borrow_mut().right = left.clone();
                }
                _ => parent.clone().unwrap().borrow_mut().left = left.clone(),
            }
        }

        left.clone().unwrap().borrow_mut().parent = parent;
    }
    pub fn is_red(node:TreeRoot<T>)->bool{
        if node.is_none(){
            //println!("uncle is none");
            return false;
        }else{
            let unwraped_node=node.clone().unwrap();
            if unwraped_node.borrow().color==NodeColor::Red{
                return true;
            }else{
                return false;
            }
        }
    }


/* 
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

*/
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
fn insert_node<T: std::cmp::PartialOrd+std::marker::Copy+ std::fmt::Debug>(node:Rc<RefCell<TreeNode<T>>>, value: T) -> TreeRoot<T>{

    println!("insert {:?}",value);
    if node.borrow().value ==value{
        //return Some(Rc::new(RefCell::new(*self)));
        return Some(node);
    }else if node.borrow().value >value{
        let left=node.borrow().left.clone();
        match left {
            Some(left_node) => {
                //let left_node = left_rc.borrow();
                insert_node(left_node,value);
            }
            None => {
                
                node.borrow_mut().left= Some(Rc::new(RefCell::new(TreeNode::new_with_parent(value, node.clone()))));
                let left=node.borrow().left.clone().unwrap();
                insert_recolor(left);
            },
        }

    }else {
        let right=node.borrow().right.clone();
        match right {
            Some(right_node) => {
                //let right_node = right_rc.borrow();
                insert_node(right_node,value);
            }
            None => {
                node.borrow_mut().right = Some(Rc::new(RefCell::new(TreeNode::new_with_parent(value, node.clone()))));
                let right=node.borrow().right.clone().unwrap();
                insert_recolor(right);
            },
        }
    }
    //return Some(Rc::new(RefCell::new(self.get_root())));
    return TreeNode::get_root(node);
}
fn insert_recolor<T: std::cmp::PartialOrd+std::marker::Copy+ std::fmt::Debug>(node:Rc<RefCell<TreeNode<T>>>){

    let parent=node.borrow().parent.clone();
    match parent {
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
                        //2.1 RR
                        if TreeNode::is_right(parent.clone())&&TreeNode::is_right(node.clone()){
                            println!("RR");
                            let uncle=grand_parent.borrow().left.clone();
                            //difference
                            //2.1.1 uncle=none||black
                            //if uncle.is_none()||uncle.clone().unwrap().borrow().color==NodeColor::Black{
                            if !TreeNode::is_red(uncle.clone()){
                                
                                //grand parent node perform left rotation
                                let grand_value=grand_parent.borrow().value;
                                println!("rotate grand, grand is {:?}",grand_value);
                                
                                TreeNode::rotate_left(grand_parent.clone());
                                //recolor parent to black and left sibling to red
                                //difference
                                let parent=node.borrow().parent.clone().unwrap();
                                TreeNode::set_black(parent.clone());
                                let left_sibling=parent.borrow().left.clone().unwrap();
                                TreeNode::set_red(left_sibling.clone());
                                

                            }
                            //2.1.2 uncle=red
                            else{
                                //set parent and uncel to black
                                TreeNode::set_black(parent.clone());
                                let unwraped_uncle=uncle.clone().unwrap();
                                TreeNode::set_black(unwraped_uncle.clone());
                                //set grand to red and recolor
                                TreeNode::set_red(grand_parent.clone());
                                insert_recolor(grand_parent.clone());
                            }
                        }
                        //2.2 LL
                        else if TreeNode::is_left(node.clone())&&TreeNode::is_left(parent.clone()){
                            println!("LL");
                            let uncle=grand_parent.borrow().right.clone();
                            //difference
                            //2.2.1 uncle=none||black
                            //if uncle.is_none()||uncle.clone().unwrap().borrow().color==NodeColor::Black{
                            if !TreeNode::is_red(uncle.clone()){
                                //grand parent node perform right rotation
                                TreeNode::rotate_right(grand_parent.clone());
                                //recolor parent to black and right sibling to red
                                //difference
                                let parent=node.borrow().parent.clone().unwrap();
                                TreeNode::set_black(parent.clone());
                                let right_sibling=parent.borrow().right.clone().unwrap();                                
                                TreeNode::set_red(right_sibling.clone());
                            }
                            //2.2.2 uncle=red
                            else{
                                //set parent and uncel to black
                                //let _=parent.borrow_mut().set_black();
                                TreeNode::set_black(parent.clone());
                                let unwraped_uncle=uncle.clone().unwrap();
                                TreeNode::set_black(unwraped_uncle.clone());
                                //set grand to red and recolor
                                //let _=grand_parent.borrow_mut().set_red();
                                TreeNode::set_red(grand_parent.clone());
                                insert_recolor(grand_parent.clone());
                            }
                        }
                        //2.3 LR
                       
                        else if TreeNode::is_left(parent.clone())&&TreeNode::is_right(node.clone()){
                            println!("LR");
                            let uncle=grand_parent.borrow().right.clone();
                            
                            //difference
                            //2.3.1 uncle=none||black
                            //if uncle.is_none()||uncle.clone().unwrap().borrow().color==NodeColor::Black{
                            if !TreeNode::is_red(uncle.clone()){
                                //left rotate parent to change LR condition into LL
                                TreeNode::rotate_left(parent.clone());
                                //now node is the parent and we take the original parent, which is the left child now as a new inserted node
                                let left_child=node.borrow().left.clone().unwrap();
                                insert_recolor(left_child.clone());
                            }
                            //2.3.2 uncle=red
                            else{
                                //set parent and uncel to black
                                
                                TreeNode::set_black(parent.clone());
                                let unwraped_uncle=uncle.clone().unwrap();
                                TreeNode::set_black(unwraped_uncle.clone());
                                //set grand to red and recolor
                                TreeNode::set_red(grand_parent.clone());
                                
                                insert_recolor(grand_parent.clone());
                            }
                        }
                        //2.4 RL
                        else if TreeNode::is_right(parent.clone())&&TreeNode::is_left(node.clone()){
                            println!("RL");
                            let uncle=grand_parent.borrow().left.clone();
                            //difference
                            //2.4.1 uncle=none||black
                            //if uncle.is_none()||uncle.clone().unwrap().borrow().color==NodeColor::Black{
                            if !TreeNode::is_red(uncle.clone()){
                                //left rotate parent to change LR condition into LL
                                TreeNode::rotate_right(parent.clone());
                                //now node is the parent and we take the original parent, which is the left child now as a new inserted node
                                let right_child=node.borrow().right.clone().unwrap();
                                insert_recolor(right_child.clone());
                            }
                            //2.4.2 uncle=red
                            else{
                                //set parent and uncel to black
                                TreeNode::set_black(parent.clone());
                                let unwraped_uncle=uncle.clone().unwrap();
                                TreeNode::set_black(unwraped_uncle.clone());
                                //set grand to red and recolor
                                TreeNode::set_red(grand_parent.clone());
                
                                insert_recolor(grand_parent.clone());
                            }
                        }
                        
                        
                    }
                    None => {
                        TreeNode::set_black(parent);

                    }
                }
            }
            //current node is the root and 

        },
        None => {
            println!("insert, node is root");
            //3. node is root
            TreeNode::set_black(node);
            
        },

    }


}

fn main() {
}
#[cfg(test)]
mod test {
    use super::*;
    #[test]
fn test_insert() {
    let mut rb_tree = RedBlackTree::new();
    rb_tree.insert(12);
    //rb_tree.print();
    rb_tree.insert(1);
    //rb_tree.print();
    rb_tree.insert(9);
    //rb_tree.print();
    rb_tree.insert(2);
    //rb_tree.print();
    rb_tree.insert(0);
    //rb_tree.print();
    rb_tree.insert(11);
    rb_tree.print();
    rb_tree.insert(7);
    rb_tree.print();
    rb_tree.insert(19);
    rb_tree.print();
    rb_tree.insert(4);
    rb_tree.print();
    rb_tree.insert(15);
    rb_tree.print();
    rb_tree.insert(18);
    rb_tree.print();
    rb_tree.insert(5);
    rb_tree.insert(14);
    rb_tree.insert(13);
    rb_tree.insert(10);
    rb_tree.insert(16);
    rb_tree.insert(6);
    rb_tree.insert(3);
    rb_tree.insert(8);
    rb_tree.insert(17);
    rb_tree.print();
    let result = RedBlackTree::is_valid_red_black_tree(rb_tree.root);
    assert_eq!(result, true);
}
}
