use criterion::{black_box, criterion_group, criterion_main, Criterion};
use rand::prelude::*;
use BinaryTrees::avltree::*;
use BinaryTrees::prelude::*;
use BinaryTrees::rbtree::*;
use BinaryTrees::tree::*;



fn bench_insert_test_avl(array:[i32;5],index:usize) {
    let tree_size = array[index];
    let mut tree = AVLTree::new();
    for i in 0..tree_size {
        tree.insert(i);
    }
}

fn bench_search_test_avl(tree:AVLTree<i32>,tree_size:i32) {
    for i in 0..tree_size/10 {
        tree.search(i);
    }
}

fn bench_insert_test_rbt(array:[i32;5],index:usize) {
    let tree_size = array[index];
    let mut tree = RedBlackTree::new();
    for i in 0..tree_size {
        tree.insert(i);
    }
}

fn bench_search_test_rbt(tree:RedBlackTree<i32>,tree_size:i32) {

    for i in 0..tree_size/10 {
        tree.search(i);

    }
}

fn avl_benchmark_insert(c: &mut Criterion) {
    let array =[10000, 40000, 70000, 100000, 130000];
    let array_index = 1; // 0 to 4 for different size test, change this manually for 5 tests
    println!("Avl benchmark insert test with size {}",array[array_index]);
    c.bench_function("Avl_insert_benchmark", |b| b.iter(|| bench_insert_test_avl(black_box(array),array_index)));
}

fn avl_benchmark_search(c: &mut Criterion) {
    let array =[10000, 40000, 70000, 100000, 130000];
    let array_index = 1; // 0 to 4 for different size test, change this manually for 5 tests
    println!("Avl benchmark search test with size {}",array[array_index]);

    let mut tree = AVLTree::new();
    let tree_size = array[array_index];
    for i in 0..tree_size {
        tree.insert(i);
    }

    c.bench_function("Avl_search_benchmark", |b| b.iter(|| bench_search_test_avl(black_box(tree.clone()),tree_size)));
}

fn rbt_benchmark_insert(c: &mut Criterion) {
    let array =[10000, 40000, 70000, 100000, 130000];
    let array_index = 1; // 0 to 4 for different size test, change this manually for 5 tests
    println!("RedBlack Tree benchmark insert test with size {}",array[array_index]);
    c.bench_function("rbt_insert_benchmark", |b| b.iter(|| bench_insert_test_rbt(black_box(array),array_index)));
}

fn rbt_benchmark_search(c: &mut Criterion) {
    let array =[10000, 40000, 70000, 100000, 130000];
    let array_index = 1; // 0 to 4 for different size test, change this manually for 5 tests
    println!("RedBlack Tree benchmark search test with size {}",array[array_index]);

    let mut tree = RedBlackTree::new();
    let tree_size = array[array_index];
    for i in 0..tree_size {
        tree.insert(i);
    }

    c.bench_function("rbt_search_benchmark", |b| b.iter(|| bench_search_test_rbt(black_box(tree.clone()),tree_size)));
}

criterion_group!(benches, avl_benchmark_insert,avl_benchmark_search, rbt_benchmark_insert, rbt_benchmark_search);
//criterion_group!(benches,avl_benchmark_search,rbt_benchmark_search);

criterion_main!(benches);






