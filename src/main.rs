struct Node {
    value: String,
    next_node: Option<Box<Node>>
}


fn add_node(head_node: &mut Node, new_node: Node) {
    let mut prev = head_node;
    loop {
        if let Some(node) = &mut prev.next_node {
            prev = node;
        } else {
            prev.next_node = Some(Box::new(new_node));
            break;
        }
    }
}

fn get_mut(head_node: &mut Node, i: usize) -> &mut Node {
    let mut prev = head_node;
    for p in 0..(i+1) {
        match &mut prev.next_node {
            Some(node) => prev = node,
            None => return prev
        }
    }
    prev
}

fn get(head_node: &Node, i: usize) -> &Node {
    let mut prev = head_node;
    for p in 0..(i+1) {
        match &prev.next_node {
            Some(node) => prev = node,
            None => return prev
        }
    }
    prev
}

fn delete_node(head_node: Node, i: usize) {
}

fn insert_node(head_node: Node) {}

fn main() {}




