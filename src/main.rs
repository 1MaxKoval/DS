mod master_slave;
mod linked_list;
use master_slave::run;

struct Node {
    value: String,
    next_node: Option<Box<Node>>
}

struct LinkedList {}


fn add_node(head_node: &mut Node, new_node: Node) {
    let mut prev = head_node;
    loop {
        let next_node = &mut prev.next_node;
        if let Some(node) = next_node {
            prev = node;
        } else {
            *next_node = Some(Box::new(new_node));
            break;
        }
    }
}

fn get_mut(head_node: &mut Node, i: usize) -> &mut Node {
    let mut current = Some(head_node);
    for _ in 0..(i+1) {
        if let Some(node) = current {
            current = node.next_node.as_deref_mut();
        } else { break; }
    }

    match current {
        Some(node) => return node,
        None => panic!("cannot happen")
    }
}


fn get(head_node: &Node, i: usize) -> &Node {
    let mut prev = head_node;
    for _ in 0..(i+1) {
        match &prev.next_node {
            Some(node) => prev = node,
            None => return prev
        }
    }
    prev
}

fn delete_node(head_node: Node, i: usize) {
}

fn insert_node(head_node: Node) {
}

fn main() {
    run();
}




