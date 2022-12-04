use std::cell::RefCell;
use std::rc::Rc;
use std::cmp::max;

use std::fmt::{Debug, Display};

pub enum Direction{
    Left,
    Right
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



pub trait TreeTrait<T: Ord+Copy+Debug+Display, TreeNode: TreeNodeTrait<T>>{

    fn root(&self)->Option<Rc<RefCell<TreeNode>>>;

    fn insert(&mut self, value: T)->bool;
    fn delete(&mut self, value: T)->Option<T>;

    fn count_leaves(&self)->u32{
        if self.root().is_some(){
            return self.root().unwrap().borrow().count_leaves();
        }
        return self.DEFAULT_LEAF_NUM();
    }
    fn is_empty(&self)->bool{
        self.root().is_none()
    }
    fn print(&self, verbose: bool){
        let root = self.root();
        if root.is_none() {
            println!("Empty tree!");
            return;
        }
        if verbose{
            root.clone().unwrap().borrow().print_tree_as_fmt("  ".to_string());
        }
        root.unwrap().borrow().print_structure();
    }

    fn height(&self)->u32{
        match self.root(){
            None => self.DEFAULT_HEIGHT_NUM(),
            Some(node) => node.borrow().get_height(),
        }
    }

    fn in_order_traverse(&self)->Vec<T>{
        let mut result = Vec::<T>::new();
        let root = self.root();
        if root.is_some(){
            root.unwrap().borrow().inorder(&mut result);
        }
        return result;
    }

    fn check_valid(&self)->bool;

    // assocated constants
    fn DEFAULT_LEAF_NUM(&self)->u32{
        0 as u32
    }

    fn DEFAULT_HEIGHT_NUM(&self)->u32{
        0 as u32
    }
}

pub trait TreeNodeTrait<T: Ord+Copy+Debug+Display>{
    fn left(&self)->Option<Rc<RefCell<Self>>>;
    fn right(&self)->Option<Rc<RefCell<Self>>>;
    fn parent(&self)->Option<Rc<RefCell<Self>>>;
    fn value(&self)->T;

    fn get_min(&self)->T{
        match &self.left(){
            None=>self.value(),
            Some(nd)=>{
                nd.borrow().get_min()
            }
        }
    }

    fn get_direction_to_parent(&self)->Direction{
        assert!(self.parent().is_some());
        let p = self.parent().unwrap();
        let parent = p.borrow();
        assert!(parent.value() != self.value());
        if parent.value() > self.value(){
            Direction::Left
        }
        else{
            Direction::Right
        }
    }

    fn get_sibling(&self)->Option<Rc<RefCell<Self>>>{
        if self.parent().is_none(){
            return None;
        }
        let p = self.parent().unwrap();
        let direc = self.get_direction_to_parent();
        match direc{
            Direction::Left=>p.borrow().right().clone(),
            Direction::Right=>p.borrow().left().clone(),
        }
    }

    fn is_leaf(&self)->bool{
        self.left().is_none() && self.right().is_none()
    }

    fn print_tree_as_fmt(&self, ident: String){
        println!("{}",self.fmt_info());

        if self.left().is_some() {
            print!("{}left: ", ident);
            self.left().unwrap().borrow().print_tree_as_fmt(ident.clone()+"  ");
        }

        if self.right().is_some() {
            print!("{}right: ", ident);
            self.right().unwrap().borrow().print_tree_as_fmt(ident.clone()+"  ");
        }
    }

    fn inorder(&self, result: &mut Vec<T>) {
        if self.left().is_some() {
            self.left().unwrap().borrow().inorder(result);
        }
        result.push(self.value());
        if self.right().is_some() {
            self.right().unwrap().borrow().inorder(result);
        }
    }

    fn get_height(&self)->u32{
        let left_height = if let Some(ln)=self.left().clone(){
            ln.borrow().get_height()
        }
        else{
            0
        };
        let right_height = if let Some(rn)=self.right().clone(){
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

        if self.left().is_some(){
            container[row_index+1][column_index-gap] = String::from("/");
            let left_child = self.left().unwrap();
            left_child.borrow().print_structure_helper(row_index+2, column_index-gap*2, container, height);
        }

        if self.right().is_some(){
            container[row_index+1][column_index+gap] = String::from("\\");
            let right_child = self.right().unwrap();
            right_child.borrow().print_structure_helper(row_index+2, column_index+gap*2, container, height);
        }
    }

    fn structure_info(&self)->String;

    fn fmt_info(&self)->String;

    fn count_leaves(&self)->u32{
        if self.is_leaf(){
            return 1;
        }
        let left_leaves = if let Some(l)=self.left(){
            l.borrow().count_leaves()
        }else{ 0 };
        let right_leaves = if let Some(r)=self.right(){
            r.borrow().count_leaves()
        }else{ 0 };
        left_leaves + right_leaves
    }
}


