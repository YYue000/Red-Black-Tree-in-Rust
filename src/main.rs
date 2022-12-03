use std::cell::RefCell;
use std::rc::Rc;
use std::cmp::max;

use std::fmt::{Debug, Display};

#[derive(Clone, Debug, PartialEq, Copy)]
enum NodeColor {
    Red,
    Black, 
}
enum Direction{
    Left,
    Right
}

#[derive(Clone, Debug, PartialEq)]
struct TreeNode<T: Ord+Copy+Debug+Display> {
    pub color: NodeColor,
    pub value: T,
    pub parent: TreeRoot<T>,
    left: TreeRoot<T>, 
    right: TreeRoot<T>
}
type TreeRoot<T> = Option<Rc<RefCell<TreeNode<T>>>>;
struct RedBlackTree<T: Ord+Copy+Debug+Display>{
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



impl<T: Ord+Copy+Debug+Display> RedBlackTree<T>{
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
    pub fn is_empty(&self)->bool{}*/
    pub fn print(&self, verbose: bool){
        if self.root.is_none() {
            println!("Empty tree!");
            return;
        }
        if verbose{
            self.root.clone().unwrap().borrow().print_tree_as_fmt("  ".to_string());
        }
        self.root.clone().unwrap().borrow().print_structure();
    }

    fn height(&self)->u32{
        match self.root.clone(){
            None => 0,
            Some(node) => node.borrow().get_height(),
        }
    }


    pub fn in_order_traverse(&self)->Vec<T>{
        let mut result = Vec::<T>::new();
        if self.root.is_some(){
            self.root.clone().unwrap().borrow().inorder(&mut result);
        }
        return result;
    }

    pub fn rotate(parent: &TreeRoot<T>, child: &TreeRoot<T>){
        let p = parent.clone().unwrap();
        let c = child.clone().unwrap();

        let node_direction = c.borrow().get_direction_to_parent(); 

        let grad = p.borrow().parent.clone();
        if grad.is_some(){
            let p_direct = p.borrow().get_direction_to_parent();
            match p_direct{
                Direction::Left=>{
                    grad.clone().unwrap().borrow_mut().left = child.clone();
                },
                Direction::Right=>{
                    grad.clone().unwrap().borrow_mut().right = child.clone();
                }
            }
        }
        match node_direction{
            Direction::Left=>{
                //rotate right
                let gc = c.borrow().right.clone();
                p.borrow_mut().left = gc.clone();
                c.borrow_mut().right = parent.clone();
                if gc.is_some(){
                    gc.clone().unwrap().borrow_mut().parent = parent.clone();
                }
            },
            Direction::Right=>{
                //rotate left
                let gc = c.borrow().left.clone();
                p.borrow_mut().right = gc.clone();
                c.borrow_mut().left = parent.clone();
                if gc.is_some(){
                    gc.clone().unwrap().borrow_mut().parent = parent.clone();
                }

            },
        }
        p.borrow_mut().parent = child.clone();
        c.borrow_mut().parent = grad.clone();

    }

    pub fn check_color(&self)->bool{
        if self.root.is_none(){
            return true;
        }
        let height_option = self.root.clone().unwrap().borrow().check_color();
        return height_option.is_some();
    }
}

impl<T: Ord+Copy+Debug+Display> TreeNode <T>{

    fn get_min(&self)->T{
        match &self.left{
            None=>self.value,
            Some(nd)=>{
                nd.borrow().get_min()
            }
        }
    }

    fn get_child_delete_helper(&self)->(TreeRoot<T>, Direction){
        // one child or no child
        if self.left.is_some(){
            return (self.left.clone(), Direction::Left);
        }
        if self.right.is_some(){
            return (self.right.clone(), Direction::Right);
        }
        return (None, Direction::Left);
    }

    fn replace_current_with_unique_child_delete_helper(&mut self, child: &TreeRoot<T>)->Option<TreeRoot<T>>{
        // deal nodes with 1 or 0 child
        assert!(!(self.left.is_some()&&self.right.is_some()));
        let direction = self.get_direction_to_parent();
        if child.is_some(){
            child.clone().unwrap().borrow_mut().parent = self.parent.clone();
        }
        let ret = match self.parent.clone(){
            Some(parent)=>{
                match direction{
                    Direction::Left=>parent.borrow_mut().left = self.left.clone(),
                    Direction::Right=>parent.borrow_mut().right = self.right.clone()
                };
                None
            },
            None=>Some(child.clone())
        };
        self.parent = None;
        self.left = None;
        self.right = None;
        return ret;
    }

    fn delete_node(&mut self)->Option<TreeRoot<T>>{
        let child = match self.right.clone(){
            None=>self.left.clone(),
            Some(_)=>self.right.clone()
        };
        self.replace_current_with_unique_child_delete_helper(&child)
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

    fn is_leaf(&self)->bool{
        self.left.is_none() && self.right.is_none()
    }

    fn print_tree_as_fmt(&self, ident: String){
        println!(
            "(Color: {:?}, Value: {:?}, Is Leaf: {:?})",
            self.color,
            self.value,
            self.is_leaf(),
        );

        if self.left.is_some() {
            print!("{}left: ", ident);
            self.left.clone().unwrap().borrow().print_tree_as_fmt(ident.clone()+"  ");
        }

        if self.right.is_some() {
            print!("{}right: ", ident);
            self.right.clone().unwrap().borrow().print_tree_as_fmt(ident.clone()+"  ");
        }
    }

    fn inorder(&self, result: &mut Vec<T>) {
        if self.left.is_some() {
            self.left.clone().unwrap().borrow().inorder(result);
        }
        result.push(self.value);
        if self.right.is_some() {
            self.right.clone().unwrap().borrow().inorder(result);
        }
    }

    fn check_color(&self)->Option<usize>{
        let left_height = if let Some(ln)=self.left.clone(){
            ln.borrow().check_color()
        }
        else{
            Some(0)
        };
        let right_height = if let Some(rn)=self.right.clone(){
            rn.borrow().check_color()
        }
        else{
            Some(0)
        };
        match (left_height, right_height){
            (Some(lh), Some(rh))=>{
                if lh != rh{
                    return None;
                }
                match &self.color{
                    NodeColor::Red => Some(lh),
                    NodeColor::Black => Some(lh+1),
                } 
            },
            _=>None
        }
    }

    fn get_height(&self)->u32{
        let left_height = if let Some(ln)=self.left.clone(){
            ln.borrow().get_height()
        }
        else{
            0
        };
        let right_height = if let Some(rn)=self.right.clone(){
            rn.borrow().get_height()
        }
        else{
            0
        };

        return max(left_height, right_height)+1;
    }

    fn print_structure(&self){
        let height = self.get_height() as usize;

        let array_height = height*2-1;
        let array_width = (2 << (height-2))*3+1 as usize;
        let mut container_raw = vec![String::from(" "); array_width*array_height];
        let mut container_base: Vec<_> = container_raw.as_mut_slice().chunks_mut(array_width).collect();
        let container: &mut [&mut [String]] = container_base.as_mut_slice();

        self.print_structure_helper(0, array_width/2, container, height);

        for i in 0..container.len() {
            let mut line = String::new();
            let mut j = 0;
            let len = container[i].len();
            loop{
                if j >= len{
                    break;
                }
                line += &container[i][j];
                if container[i][j].chars().next().unwrap() != ' '{
                    if container[i][j].len() > 4 {
                        j += 3;
                    } 
                    else{
                        j += container[i][j].len();
                    }
                }
                else{
                    j += 1;
                }
            }
            println!("{}",line);
        }
    }

    fn print_structure_helper(&self, row_index: usize, column_index: usize,
        container: &mut [&mut [String]], height: usize){
        container[row_index][column_index] = self.structure_info();
        let curr_height = (row_index+1)/2;

        if curr_height == height {
            return;
        }
        let gap = height-curr_height-1;

        if self.left.is_some(){
            container[row_index+1][column_index-gap] = String::from("/");
            let left_child = self.left.clone().unwrap();
            left_child.borrow().print_structure_helper(row_index+2, column_index-gap*2, container, height);
        }

        if self.right.is_some(){
            container[row_index+1][column_index+gap] = String::from("\\");
            let right_child = self.right.clone().unwrap();
            right_child.borrow().print_structure_helper(row_index+2, column_index+gap*2, container, height);
        }
    }

    fn structure_info(&self)->String{
        let val = self.value.to_string();
        let cl = match self.color{
            NodeColor::Red=>"",
            NodeColor::Black=>"b"
        }.to_string();
        return val+&cl;
    }

}

fn delete_node<T: Ord+Copy+Debug+Display>(
    root: TreeRoot<T>, value: T)->(Option<T>, Option<TreeRoot<T>>){
        if root.is_none(){
            return (None, None);
        }

        let node = root.clone().unwrap();


        // return None, None if value is not in the tree
        let nd_val = node.borrow().value;
        match value{
            v if v < nd_val=>{
                let left = node.borrow().left.clone();
                match left{
                    None=>{return (None, None);}
                    Some(_)=>{
                        let left = node.borrow().left.clone();
                        return delete_node(left, value);
                    }
                }
            },
            v if v > nd_val =>{
                let right = node.borrow().right.clone();
                match right{
                    None=>{return (None, None);}
                    Some(_)=>{
                        let right = node.borrow().right.clone();
                        return delete_node(right, value);
                    }
                }
            },
            _=>{}
        };

        
        // Case0.1: No child
        // red=>just delete it
        if node.borrow().left.is_none() && node.borrow().right.is_none() && node.borrow().color == NodeColor::Red{
            let ret = node.borrow_mut().delete_node();
            return (Some(value), ret);
        }
        //else: no child&&black; two children; one child

        // Case0.2: Two children
        // => like BSTree
        if node.borrow().left.is_some() && node.borrow().right.is_some(){
            let right_min = node.borrow().right.clone().unwrap().borrow().get_min();
            let rchild = node.borrow().right.clone();
            let (_v, r) = delete_node(rchild, right_min);
            node.borrow_mut().value = right_min;
            return (Some(value), r);
        }
        // else: one child; no child && black

        // Case1: current node is red
        // Red case ends
        // Because red nodes must have 0 or 2 black children
        // else: current is black && one child; current black && no child

        // Case2: current black && one child
        // Case2.1: current is black && unique child is red
        // => Replace it with its red child
        let (child, direction) = node.borrow().get_child_delete_helper();
        if child.is_some(){
            if child.clone().unwrap().borrow().color == NodeColor::Red{
                child.clone().unwrap().borrow_mut().color = NodeColor::Black;
                let ret = node.borrow_mut().delete_node();
                return (Some(node.borrow().value), ret);
            }
            else{
                // current black && unique child black=>invalid case;
                panic!("Error! If current node is black, its unique child cannot be black!");
            }
        }
        // else: 
        // current black && no child
 
        // Case3: 
        let ret0 = delete_rebalance_helper(root.clone());
        let ret = node.borrow_mut().delete_node();
        let r = match &ret{
            None=>ret0,
            _=> ret
        };
        return (Some(value), r);
    }
fn delete_rebalance_helper<T: Ord+Copy+Debug+Display>(root: TreeRoot<T>)->Option<TreeRoot<T>> {
        if root.is_none(){
            return None;
        }

        let node = root.clone().unwrap();

        let mut new_root_ret: Option<TreeRoot<T>> = None;
        // Case3: current black && no child
        // Case3.1: child is new root
        // => node is root => finished 
        if node.borrow().parent.is_none(){
            return None; 
        }
        let direction = node.borrow().get_direction_to_parent();
        // First replace current node with its child N
        // Case3.2: sibling is red
        // =>sibling to black; parent to red; rotate 
        let mut sibling = match direction{
            Direction::Left=>node.borrow().parent.clone().unwrap().borrow_mut().right.clone().unwrap(),
            Direction::Right=>node.borrow().parent.clone().unwrap().borrow_mut().left.clone().unwrap(),
        };
        let sib_direction = direction.opposite();

        if sibling.borrow().color==NodeColor::Red{
            sibling.borrow_mut().color = NodeColor::Black;
            node.borrow().parent.clone().unwrap().borrow_mut().color = NodeColor::Red;
            RedBlackTree::rotate(&node.borrow().parent, &Some(sibling.clone()));
            if sibling.borrow().parent.is_none(){
                new_root_ret = Some(Some(sibling.clone()));
            }

            // sibling changed due to rotation
            sibling = match direction{
                Direction::Left=>node.borrow().parent.clone().unwrap().borrow().right.clone().unwrap(),
                Direction::Right=>node.borrow().parent.clone().unwrap().borrow().left.clone().unwrap(),
            };
        }

        // continue in Case3.4, 3.5, 3.6
        // else: sibling is black
        assert!(sibling.borrow().color == NodeColor::Black);
        // Case3.3&3.4: sibling black and black children or no child;
        let sib_left = sibling.borrow().left.clone();
        let sib_right = sibling.borrow().right.clone();
        if sib_left.is_some()&&sib_right.is_some()&&sib_left.clone().unwrap().borrow().color == NodeColor::Black && sib_right.clone().unwrap().borrow().color == NodeColor::Black ||
        sib_left.is_none()&&sib_right.is_none(){
            sibling.borrow_mut().color = NodeColor::Red;
            let par_color = node.borrow().parent.clone().unwrap().borrow().color;
            match par_color{
                // Case 3.3
                NodeColor::Black=>{
                    let parent = node.borrow().parent.clone();
                    let r = delete_rebalance_helper(parent);
                    return r;
                },
                // Case 3.4
                NodeColor::Red=>{
                    node.borrow().parent.clone().unwrap().borrow_mut().color = NodeColor::Black;
                    return new_root_ret;
                }
            }
        }
        //else: sibling black && either child is red 

        if sibling.borrow().left.is_some() && sibling.borrow().right.is_some() &&
            sibling.borrow().left.clone().unwrap().borrow().color == NodeColor::Red && 
            sibling.borrow().right.clone().unwrap().borrow().color == NodeColor::Red {
            panic!("not implemented");
        }

        // Case3.5: sibling close child is red
        // => rotate, change color
        let sib_close_child = match sib_direction{
            Direction::Left=>sibling.borrow().right.clone(),
            Direction::Right=>sibling.borrow().left.clone(),
        };

        if sib_close_child.is_some() && sib_close_child.clone().unwrap().borrow().color == NodeColor::Red{
            RedBlackTree::rotate(&Some(sibling.clone()), &sib_close_child);
            if sib_close_child.clone().unwrap().borrow().parent.is_none(){
                new_root_ret = Some(sib_close_child.clone());
            }
            let sp_cl = sibling.borrow().parent.clone().unwrap().borrow().color;
            sibling.borrow_mut().color = sp_cl; 
            sibling.borrow().parent.clone().unwrap().borrow_mut().color = NodeColor::Black;
            // sibling changed due to rotation
            sibling = match direction{
                Direction::Left=>node.borrow().parent.clone().unwrap().borrow().right.clone().unwrap(),
                Direction::Right=>node.borrow().parent.clone().unwrap().borrow().left.clone().unwrap(),
            };
        }


        // Case3.6: sibling distant child is red 
        // =>  rotate, change color
        let sib_dist_child = match sib_direction{
            Direction::Left=>sibling.borrow().left.clone(),
            Direction::Right=>sibling.borrow().right.clone(),
        };
        if sib_dist_child.is_some() && sib_dist_child.clone().unwrap().borrow().color == NodeColor::Red{
            sib_dist_child.clone().unwrap().borrow_mut().color = NodeColor::Black;
            sibling.borrow_mut().color = node.borrow().parent.clone().unwrap().borrow().color;
            node.borrow().parent.clone().unwrap().borrow_mut().color = NodeColor::Black;
            RedBlackTree::rotate(&node.borrow().parent, &Some(sibling.clone()));
            if sibling.borrow().parent.is_none(){
                new_root_ret = Some(Some(sibling.clone()));
            }
        }
       return  new_root_ret;
    }

fn tmp(nd: &TreeRoot<i32>, lv:i32, rv:i32,lc: &str, rc:&str)-> (Option<Rc<RefCell<TreeNode<i32>>>>, Option<Rc<RefCell<TreeNode<i32>>>>){
    let f = |s|if s == "r" {NodeColor::Red} else {NodeColor::Black};
    let left: TreeNode<i32> = TreeNode{color: f(lc),
    value:lv, parent: Some(Rc::clone(&nd.clone().unwrap())), left: None, right:None};
    let left = Some(Rc::new(RefCell::new(left)));
    let right: TreeNode<i32> = TreeNode{color: f(rc),
    value:rv, parent: Some(Rc::clone(&nd.clone().unwrap())), left: None, right:None};
    let right = Some(Rc::new(RefCell::new(right)));
    return (left, right);

}
fn main() {
    let nd:TreeNode<i32> = TreeNode{color: NodeColor::Black,
    value: 8, parent: None, left: None, right: None};
    let nd = Some(Rc::new(RefCell::new(nd)));
    let (left, right) = tmp(&nd, 2, 12, "b", "b");
    let (rl, rr) = tmp(&right, 10, 20, "r", "b");
    let (rll, rlr) = tmp(&rl, 9, 11, "b", "b");
    let (ll, lr) = tmp(&left, 1, 5, "b", "b");
    
    let mut tree: RedBlackTree<i32> = RedBlackTree{root: nd.clone()};
    {
    let _nd = tree.root.clone().unwrap();
    _nd.borrow_mut().left = left.clone();
    _nd.borrow_mut().right = right.clone();
    
    let _right = right.clone().unwrap();
    _right.borrow_mut().left = rl.clone();
    _right.borrow_mut().right = rr.clone();
    let _left = left.clone().unwrap();
    _left.borrow_mut().left = ll.clone();
    _left.borrow_mut().right = lr.clone();
    let _rl = rl.clone().unwrap();
    _rl.borrow_mut().left = rll.clone();
    _rl.borrow_mut().right = rlr.clone();
    }
   tree.print(false);
   println!("{:?} {:?}", tree.check_color(), tree.in_order_traverse());

   let d = tree.delete(2);
   println!("{:?}",d);
   tree.print(false);
   println!("{:?} {:?}", tree.check_color(), tree.in_order_traverse());
    println!("{:?}",tree.height());
}