// Each node owns its children through Box, so the tree's size is known and each node is freed with its subtree.
struct Node {
    value: i32,
    left: Option<Box<Node>>,
    right: Option<Box<Node>>,
}

pub struct Tree {
    root: Option<Box<Node>>,
}

// Inserts into the slot that is either empty or holds a subtree. A duplicate value does nothing.
fn insert_into(slot: &mut Option<Box<Node>>, value: i32) {
    match slot {
        None => {
            *slot = Some(Box::new(Node {
                value,
                left: None,
                right: None,
            }));
        }
        Some(node) => {
            if value < node.value {
                insert_into(&mut node.left, value);
            } else if value > node.value {
                insert_into(&mut node.right, value);
            }
        }
    }
}

impl Tree {
    pub fn new() -> Tree {
        Tree { root: None }
    }

    pub fn insert(&mut self, value: i32) {
        insert_into(&mut self.root, value);
    }

    pub fn contains(&self, value: i32) -> bool {
        let mut current = &self.root;
        while let Some(node) = current {
            if value == node.value {
                return true;
            }
            current = if value < node.value {
                &node.left
            } else {
                &node.right
            };
        }
        false
    }

    pub fn depth(&self) -> usize {
        fn depth_of(node: &Option<Box<Node>>) -> usize {
            match node {
                None => 0,
                Some(n) => 1 + depth_of(&n.left).max(depth_of(&n.right)),
            }
        }
        depth_of(&self.root)
    }
}

fn main() {
    let mut tree = Tree::new();
    for value in [5, 3, 8, 1, 4] {
        tree.insert(value);
    }

    println!("contains 4: {}", tree.contains(4));
    println!("contains 7: {}", tree.contains(7));
    println!("depth: {}", tree.depth());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_tree_has_depth_zero() {
        assert_eq!(Tree::new().depth(), 0);
    }

    #[test]
    fn finds_inserted_values_only() {
        let mut tree = Tree::new();
        tree.insert(5);
        tree.insert(3);
        assert!(tree.contains(3));
        assert!(!tree.contains(4));
    }

    #[test]
    fn duplicates_are_ignored() {
        let mut tree = Tree::new();
        tree.insert(5);
        tree.insert(5);
        assert_eq!(tree.depth(), 1);
    }
}
