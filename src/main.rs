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

#[derive(Debug)]
struct AvlTreeSetIter<'a, T: Ord> {
    prev_nodes: Vec<&'a AvlNode<T>>,
    current_tree: &'a AvlTree<T>,
}

impl<'a, T: 'a + Ord> Iterator for AvlTreeSetIter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
    loop {
        match *self.current_tree {
            None => match self.prev_nodes.pop() {
                None => {
                    return None;
                }

                Some(ref prev_node) => {
                    self.current_tree = &prev_node.right;

                    return Some(&prev_node.value);
                }
            },

            Some(ref current_node) => {
                if current_node.left.is_some() {
                    self.prev_nodes.push(&current_node);
                    self.current_tree = &current_node.left;

                    continue;
                }

                if current_node.right.is_some() {
                    self.current_tree = &current_node.right;

                    return Some(&current_node.value);
                }

                self.current_tree = &None;

                return Some(&current_node.value);
            }
        }
    }
}
}

// Addition of lifetime parameter for the set
impl<'a, T: 'a + Ord> AvlTreeSet<T> {
    fn iter(&'a self) -> AvlTreeSetIter<'a, T> {
        AvlTreeSetIter {
            prev_nodes: Vec::new(),
            current_tree: &self.root,
        }
    }
}

fn main() {
    println!("Hello, world!");
    let mut set = AvlTreeSet::new();

    for i in (1..4 as usize).rev() {
        set.insert(i);
    }

    let mut iter = set.iter();
    assert_eq!(iter.next(), Some(&1));
    assert_eq!(iter.next(), Some(&2));
    assert_eq!(iter.next(), Some(&3));
    assert_eq!(iter.next(), None);
}

