// use std::ptr;
use std::mem::swap;
use std::process::exit;
use std::rc::{Rc, Weak};
use std::cell::RefCell;

const INF: i32 = -2000000; // initialize the value of fantom node

type Datatype = i32; // determine the datatype of the nodes

#[derive(Debug)]
struct Node {
    value: Datatype,
    color: i32, // Color: 1 for red, 0 for black
    parent: Option<Weak<RefCell<Node>>>,
    left: Option<Rc<RefCell<Node>>>,
    right: Option<Rc<RefCell<Node>>>,
}

#[derive(Debug)]
struct SavedNodeInfo {
    saved_node: Option<Rc<RefCell<Node>>>,
    is_left_or_right_child: i32,
    deleted_node_color: i32,
}

impl Node {
    // Initialize a new node
    fn new(x: Datatype, c: i32) -> Rc<RefCell<Node>> {
        Rc::new(RefCell::new(Node {
            value: x,
            color: c,
            parent: None,
            left: None,
            right: None,
        }))
    }
}

fn preorder(root: Option<Rc<RefCell<Node>>>) {
    if let Some(node) = root {
        let node_ref = node.borrow();
        print!("{}-{}", node_ref.value, node_ref.color);
        if let Some(parent) = &node_ref.parent {
            if let Some(parent_node) = parent.upgrade() {
                print!("-p:{} ", parent_node.borrow().value);
            }
        } else {
            print!("-p:N ");
        }
        preorder(node_ref.left.clone());
        preorder(node_ref.right.clone());
    }
}

fn check_red_black_tree(root: Option<Rc<RefCell<Node>>>) -> i32 {
    if root.is_none() {
        return 0;
    }
    let root_ref = root.as_ref().unwrap().borrow();

    // Root node must not be red
    if root_ref.parent.is_none() && root_ref.color == 1 {
        return -1;
    }

    // Recursively check the left and right substrees
    let leftroute = check_red_black_tree(root_ref.left.clone());
    if leftroute == -1 {
        return -1;
    }
    let rightroute = check_red_black_tree(root_ref.right.clone());
    if rightroute == -1 {
        return -1;
    }

    // Check red node violation (no two consecutive red nodes)
    if root_ref.color == 1 {
        if root_ref.left.as_ref().map_or(false, |left| left.borrow().color == 1)
            || root_ref.right.as_ref().map_or(false, |right| right.borrow().color == 1)
        {
            return 1;
        }
    }

    // Ensure black node counts are the same for left and right subtrees
    if leftroute != rightroute {
        return -1;
    }

    // Update the black height based on the current node's color
    if root_ref.color == 1 {
        leftroute
    } else {
        leftroute + 1
    }
}

fn insert_bst(
    root: Option<Rc<RefCell<Node>>>,
    parent: Option<Rc<RefCell<Node>>>,
    x: Datatype,
    color: i32,
) -> Option<Rc<RefCell<Node>>> {
    if let Some(node) = root {
        {
            let mut node_ref = node.borrow_mut();
            if x < node_ref.value {
                node_ref.left = insert_bst(node_ref.left.clone(), Some(node.clone()), x, color);
            } else if x > node_ref.value {
                node_ref.right = insert_bst(node_ref.right.clone(), Some(node.clone()), x, color);
            }
        }
        Some(node)
    } else {
        let new_node = Node::new(x, color);
        if let Some(parent_node) = parent {
            new_node.borrow_mut().parent = Some(Rc::downgrade(&parent_node));
        }
        Some(new_node)
    }
}

fn main() {
    let mut root: Option<Rc<RefCell<Node>>> = None;

    // Insert nodes into the BST
    root = insert_bst(root, None, 50, 0); // Black root
    root = insert_bst(root, None, 30, 1); // Red node
    root = insert_bst(root, None, 70, 1); // Red node

    // Test preorder traversal
    println!("Preorder traversal:");
    preorder(root.clone());
    println!();

    // Check red-black tree properties
    let check_result = check_red_black_tree(root.clone());
    if check_result == -1 {
        println!("The tree does not satisfy red-black tree properties.");
    } else {
        println!("The tree satisfies red-black tree properties. Black height: {}", check_result);
    }
}