mod master_slave;
mod linked_list;
use master_slave::run;
use linked_list::LinkedList;

fn main() {
}

fn linked_list() {
    let a = "first".to_string();
    let b = "second".to_string();
    let c = "third".to_string();
    let d = "inserted-start".to_string();
    let e = "inserted-mid".to_string();
    let f = "inserted-last".to_string();
    let mut ll = LinkedList::new(a);
    println!("{} {}", ll, ll.length);
    ll.push(b);
    println!("{} {}", ll, ll.length);
    ll.push(c);
    println!("{} {}", ll, ll.length);
    ll.pop();
    println!("{} {}", ll, ll.length);
    ll.insert(d, 0);
    println!("{} {}", ll, ll.length);
    ll.insert(e, 2);
    println!("{} {}", ll, ll.length);
    ll.insert(f, 3);
    println!("{} {}", ll, ll.length);
}


