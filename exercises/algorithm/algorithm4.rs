/*
    binary_search tree
    This problem requires you to implement a basic interface for a binary tree
*/

use std::cmp::Ordering;
use std::fmt::Debug;

#[derive(Debug)]
struct TreeNode<T>
where
    T: Ord,
{
    value: T,
    left: Option<Box<TreeNode<T>>>,
    right: Option<Box<TreeNode<T>>>,
}

#[derive(Debug)]
struct BinarySearchTree<T>
where
    T: Ord,
{
    root: Option<Box<TreeNode<T>>>,
}

impl<T> TreeNode<T>
where
    T: Ord,
{
    fn new(value: T) -> Self {
        TreeNode {
            value,
            left: None,
            right: None,
        }
    }
}

impl<T> BinarySearchTree<T>
where
    T: Ord,
{
    fn new() -> Self {
        BinarySearchTree { root: None }
    }

    // Insert a value into the BST
    fn insert(&mut self, value: T) {
        //TODO
        match self.root {
            None => self.root = Some(Box::new(TreeNode::<T>::new(value))),
            Some(_) => {
                let mut cur = &mut self.root;
                while let Some(mut node) = cur.take() {
                    let target_ptr: *mut _ = if value < node.value {
                        &mut node.left as *mut _ // 裸指针，不触发借用检查
                    } else if value > node.value {
                        &mut node.right as *mut _
                    } else {
                        // 相等，把树放回去直接返回
                        *cur = Some(node);
                        return;
                    };

                    // 安全地再把树放回去
                    *cur = Some(node);

                    // 继续向下走
                    let target_ref: &mut Option<_> = unsafe { &mut *target_ptr }; // 回到 &mut Option<Box<_>>
                    if target_ref.is_some() {
                        cur = target_ref;
                    } else {
                        *target_ref = Some(Box::new(TreeNode::new(value)));
                        return;
                    }
                }
            }
        }
    }

    // Search for a value in the BST
    fn search(&self, value: T) -> bool {
        //TODO
        match self.root {
            None => return false,
            Some(_) => {
                let mut cur = &self.root;
                while let Some(node) = &cur {
                    let target_ptr = if value > node.value {
                        &node.right as *const _
                    } else if value < node.value {
                        &node.left as *const _
                    } else {
                        return true;
                    };

                    let target_ref: &Option<Box<_>> = unsafe { &*target_ptr };
                    match target_ref {
                        None => {
                            return false;
                        }
                        Some(_) => {
                            cur = target_ref;
                        }
                    }
                }
            }
        }
        return false;
    }
}

impl<T> TreeNode<T>
where
    T: Ord,
{
    // Insert a node into the tree
    fn insert(&mut self, value: T) {
        //TODO
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_insert_and_search() {
        let mut bst = BinarySearchTree::new();

        assert_eq!(bst.search(1), false);

        bst.insert(5);
        bst.insert(3);
        bst.insert(7);
        bst.insert(2);
        bst.insert(4);

        assert_eq!(bst.search(5), true);
        assert_eq!(bst.search(3), true);
        assert_eq!(bst.search(7), true);
        assert_eq!(bst.search(2), true);
        assert_eq!(bst.search(4), true);

        assert_eq!(bst.search(1), false);
        assert_eq!(bst.search(6), false);
    }

    #[test]
    fn test_insert_duplicate() {
        let mut bst = BinarySearchTree::new();

        bst.insert(1);
        bst.insert(1);

        assert_eq!(bst.search(1), true);

        match bst.root {
            Some(ref node) => {
                assert!(node.left.is_none());
                assert!(node.right.is_none());
            }
            None => panic!("Root should not be None after insertion"),
        }
    }
}
