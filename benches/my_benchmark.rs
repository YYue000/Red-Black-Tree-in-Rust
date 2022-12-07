use criterion::{black_box, criterion_group, criterion_main, Criterion};
use rand::prelude::*;
use BTrees::avltree::*;
use BTrees::prelude::*;
use BTrees::rbtree::*;
use BTrees::tree::*;



fn bench_test_avl(array:&[i32]) {
    for tree_size in array {
        let mut tree = AVLTree::new();
        for i in 0..*tree_size {
            tree.insert(i);
        }
        //assert_eq!(tree.len(), *tree_size as usize);
        //assert!(tree.is_balanced());
        /*
        for i in 0..*tree_size/10 {
            tree.search_node(i);
        }

         */
    }
}

fn avl_benchmark(c: &mut Criterion) {
    c.bench_function("Avl_insert_benchmark", |b| b.iter(|| bench_test_avl(black_box(&[10000, 40000, 70000, 100000, 130000]))));
}

// criterion_group!(benches, avl_benchmark);
// criterion_main!(benches);




fn bench_test_rbt(array:&[i32]) {
    for tree_size in array {
        let mut tree = RedBlackTree::new();
        for i in 0..*tree_size {
            tree.insert(i);
        }
        //assert_eq!(tree.len(), *tree_size as usize);

        /*
        for i in 0..*tree_size/10 {
            tree.search_node(i);
        }

         */
    }
}
fn rbt_benchmark(c: &mut Criterion) {
    c.bench_function("Rbt_insert_benchmark", |b| b.iter(|| bench_test_rbt(black_box(&[10000, 40000, 70000, 100000, 130000]))));
}
//
criterion_group!(benches, rbt_benchmark);
criterion_main!(benches);

