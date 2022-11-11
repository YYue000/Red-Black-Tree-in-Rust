
pub mod Avl_tree{
    #[derive(Debug, PartialEq, Clone)]
    pub struct AvlNode<T: Ord> {
        pub value: T,
        pub left: AvlTree<T>,
        pub right: AvlTree<T>,
    }

    type AvlTree<T> = Option<Box<AvlNode<T>>>; // What is Box?

    #[derive(Debug, PartialEq, Clone)]
    pub struct AvlTreeSet<T: Ord> {
        root: AvlTree<T>,
    }

    impl<T: Ord> AvlTreeSet<T> {
        fn new() -> Self {
            Self { root: None }
        }
    }

}