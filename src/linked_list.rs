use std::fmt;

struct Node<T> {
    value: T,
    next_node: Option<Box<Node<T>>>
}

impl fmt::Display for Node<String> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({})", self.value)
    }
}

pub struct LinkedList<T> {
    head_node: Option<Box<Node<T>>>,
    pub length: usize
}

impl fmt::Display for LinkedList<String> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut current = &self.head_node;
        let mut r = "".to_string();
        while let Some(node) = current {
            if let Some(_) = &node.next_node {
                r.push_str(&format!(" {},", node));
            } else { r.push_str(&format!(" {} ", node)); }
            current = &node.next_node;
        }
        write!(f, "[{}]", r)
    }
}

impl<T> LinkedList<T> {

    pub fn new(data: T) -> Self {
        let hn = Node {
            value: data,
            next_node: None
        };
        LinkedList {
            head_node: Some(Box::new(hn)),
            length: 1
        }
    }

    pub fn push(&mut self, data: T) { 
        let new_node = Node {
            value: data,
            next_node: Option::None
        };
        match &mut self.head_node {
            Some(node) => {
                let mut prev = node;
                loop {
                    let next_node = &mut prev.next_node;
                    if let Some(node) = next_node {
                        prev = node;
                    } else {
                        *next_node = Some(Box::new(new_node));
                        break;
                    }
                }
            },
            None => {
                self.head_node = Some(Box::new(new_node));
            }
        }
        self.length += 1;
    }

    pub fn get(&self, i: usize) -> &T { 
        if i >= self.length {
            panic!("LinkedList: index out of range!")
        }
        match &self.head_node {
            Some(node) => { 
                let mut prev = node;
                for _ in 0..(i+1) {
                    match &prev.next_node {
                        Some(node) => prev = node,
                        None => return &prev.value
                    }
                }
                &prev.value
            }
            None => { panic!("No elements present in the linked list!") }
        }
    }

    pub fn pop(&mut self) {
        if self.length == 0 {
            return;
        } else {
            self.delete(self.length - 1);
        }
    }

    pub fn delete(&mut self, i: usize) {
        if i >= self.length {
            panic!("LinkedList: index out of range!");
        }
        if self.length == 0 {
            panic!("LinkedList: Cannot delete elements from an empty LinkedList!");
        }
        
        if i == 0 {
            let f = self.head_node.as_mut().unwrap().next_node.take();
            self.head_node = f;
        }
        else {
            let mut current = self.head_node.as_mut().unwrap(); // Panicked earlier anyway
            for _ in 1..i {
                current = current.next_node.as_mut().unwrap();
            }
            let follow = current.next_node.as_mut().unwrap().next_node.take();
            current.next_node = follow;
        }
        self.length -= 1
    }

    pub fn insert(&mut self, data: T, i: usize) {
        if i >= self.length {
            panic!("LinkedList: index out of range!")
        }
        if self.length == 0 {
            panic!("Linkedlist: Cannot insert a value into an empty LinkedList!")
        }

        if i == 0 {
            let n = self.head_node.take();
            let new_node = Node { value: data, next_node: n };
            self.head_node.replace(Box::new(new_node));
        }
        else {
            let mut current = self.head_node.as_mut().unwrap(); // Panicked earlier anyway
            for _ in 1..i {
                current = current.next_node.as_mut().unwrap();
            }
            let n = current.next_node.take();
            let new_node = Node { value: data, next_node: n };
            current.next_node.replace(Box::new(new_node));
        }

        self.length += 1;
    }

}

