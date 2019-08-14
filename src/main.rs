use std::cmp::Ordering;
use std::iter::{FromIterator, IntoIterator};
use itertools::equal;

type AvlTree<T> = Option<Box<AvlNode<T>>>;

#[derive(Debug, PartialEq, Clone)]
struct AvlNode<T: Ord> {
    value: T,
    left: AvlTree<T>,
    right: AvlTree<T>,
}

#[derive(Debug, PartialEq, Clone)]
struct AvlTreeSet<T: Ord> {
    root: AvlTree<T>,
}

impl<T: Ord> AvlTreeSet<T> {
    fn new() -> Self {
        Self { root: None }
    }

    fn insert(&mut self, value: T) -> bool {
        let mut current_tree = &mut self.root;
        while let Some(current_node) = current_tree {
            match current_node.value.cmp(&value) {
                Ordering::Less => {
                    current_tree = &mut current_node.right;
                }
                Ordering::Equal => {
                    return false;
                }
                Ordering::Greater => {
                    current_tree = &mut current_node.left;
                }
            }
        }

        *current_tree = Some(Box::new(AvlNode {
            value,
            left: None,
            right: None,
        }));
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

impl<T: Ord> FromIterator<T> for AvlTreeSet<T> {
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        let mut set = Self::new();

        for i in iter {
            set.insert(i);
        }

        set
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

#[cfg(test)]
extern crate quickcheck;
#[cfg(test)]
#[macro_use(quickcheck)]
extern crate quickcheck_macros;

#[cfg(test)]
mod properties {
    use super::*;
    use std::collections::BTreeSet;

    // 1. Create a random list
    #[quickcheck]
    fn iterator_parity(mut xs: Vec<usize>) -> bool {
        // 2. Create an empty AVL tree and BTree
        // 3. For each element in the list, insert it to both tree
        let avl_set = xs.iter().cloned().collect::<AvlTreeSet<_>>();
        let btree_set = xs.iter().cloned().collect::<BTreeSet<_>>();

        // 4. Both AVL and BTree iterator should be equal item per item
        equal(avl_set.iter(), btree_set.iter())
       
        //true
    }
    #[quickcheck]
    fn insert_parity(mut btree_set: BTreeSet<u8>, x: u8) -> bool {
        let mut avl_set = btree_set.iter().cloned().collect::<AvlTreeSet<_>>();

        avl_set.insert(x) == btree_set.insert(x)
    }
}
