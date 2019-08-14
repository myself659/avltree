use itertools::equal;
use std::cmp::max;
use std::cmp::Ordering;
use std::iter::{FromIterator, IntoIterator};
//type AvlTree<T> = Option<Box<AvlNode<T>>>;
use core::iter::Map;
use std::cell::RefCell;
use std::rc::Rc;

type AvlTree<'a, T> = Option<Rc<RefCell<AvlNode<'a, T>>>>;
//type AvlTree<a,T> = Option<Rc<RefCell<AvlNode<a,T>>>>;
#[derive(Debug, PartialEq, Clone)]
struct AvlNode<'a, T: Ord> {
    value: T,
    left: AvlTree<'a, T>,
    right: AvlTree<'a, T>,
    height: usize,

    parent_node: Option<&'a mut AvlNode<'a, T>>,
}

impl<'a, T: 'a + Ord> AvlNode<T> {
    fn left_height(&self) -> usize {
        self.left.as_ref().map_or(0, |left| left.height())
    }

    fn right_height(&self) -> usize {
        self.right.as_ref().map_or(0, |right| right.height())
    }

    fn update_height(&mut self) {
        self.height = 1 + max(self.left_height(), self.right_height());
    }

    fn height(&self) -> usize {
        1 + max(
            self.left.as_ref().map_or(0, |node| node.height()),
            self.right.as_ref().map_or(0, |node| node.height()),
        )
    }

    fn update_ancestors_height(&mut self) {
        self.update_height();

        let mut current_node = self;
        while let Some(parent_node) = current_node.parent_node {
            parent_node.update_height();
            current_node = parent_node;
        }
    }

    pub fn balance_factor(&self) -> i8 {
        let left_height = self.left_height();
        let right_height = self.right_height();

        if left_height >= right_height {
            (left_height - right_height) as i8
        } else {
            -((right_height - left_height) as i8)
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
struct AvlTreeSet<'a, T: 'a+Ord> {
    root: AvlTree<'a, T>,
}

impl<'a, T: 'a + Ord> AvlTreeSet<T> {
    fn new() -> Self {
        Self { root: None }
    }

    unsafe fn insert(&mut self, value: T) -> bool {
        let mut prev_ptrs = Vec::<*mut AvlNode<T>>::new();
        let mut current_tree = &mut self.root;
        while let Some(current_node) = current_tree {
            prev_ptrs.push(&mut **current_node);
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

        for node_ptr in prev_ptrs.into_iter().rev() {
            let node = unsafe { &mut *node_ptr }; // Converting a mutable pointer back to a reference
            node.update_height();
        }
        true
    }
    
    fn iter(&'a self) -> impl Iterator<Item = &'a T> + 'a {
        self.node_iter().map(|node| &node.value)
    }
    

    fn node_iter(&'a self) -> impl Iterator<Item = &'a AvlNode<T>> + 'a {
        AvlTreeSetNodeIter {
            prev_nodes: Vec::default(),
            current_tree: &self.root,
        }
    }
}

#[derive(Debug)]
struct AvlTreeSetNodeIter<'a, T: Ord> {
    prev_nodes: Vec<&'a AvlNode<'a, T>>,
    current_tree: &'a AvlTree<'a, T>,
}

#[derive(Debug)]
struct AvlTreeSetIter<'a, T: Ord> {
    prev_nodes: Vec<&'a AvlNode<'a, T>>,
    current_tree: &'a AvlTree<'a,T>,
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
/*
impl<'a, T: 'a + Ord> AvlTreeSet<T> {
    fn iter(&'a self) -> AvlTreeSetIter<'a, T> {
        AvlTreeSetIter {
            prev_nodes: Vec::new(),
            current_tree: &self.root,
        }
    }
}
*/

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
