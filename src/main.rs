use std::cmp::Ordering;

type AvlTree<T> = Option<Box<AvlNode<T>>>;

#[derive(Debug, PartialEq, Clone)]
struct AvlNode<T:Ord> {
    value: T, 
    left: AvlTree<T>,
    right: AvlTree<T>,
}



#[derive(Debug, PartialEq,Clone)]
struct AvlTreeSet<T:Ord> {
    root: AvlTree<T>,
}

impl<T:Ord>  AvlTreeSet<T>{
    fn new() ->Self{
        Self{ root: None}
    }

    fn insert(&mut self, value:T) -> bool{
        let mut current_tree = &mut self.root;
        while let Some(current_node) = current_tree{
            match current_node.value.cmp(&value){
                Ordering::Less=>{
                    current_tree = &mut current_node.right;
                }
                Ordering::Equal =>{
                    return  false;
                }
                Ordering::Greater =>{
                    current_tree = &mut  current_node.left;
                }
            }
        }

        *current_tree = Some(Box::new(AvlNode{
            value,
            left:None,
            right:None,})); 
        true
    }
}

fn main() {
    println!("Hello, world!");
    let mut set = AvlTreeSet::new();

assert!(set.insert(1));   // Insert new value
assert!(!set.insert(1));  // Should not insert existing value

assert!(set.insert(2));   // Insert another new value
assert_eq!(               // Checking the tree structure
    set.root,
    Some(Box::new(AvlNode {
        value: 1,
        left: None,
        right: Some(Box::new(AvlNode {
            value: 2,
            left: None,
            right: None
        })),
    }))
);
}

