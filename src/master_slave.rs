use std::cell::RefCell;
use std::rc::Rc;

struct Master {
    slaves: RefCell<Option<Vec<Rc<Slave>>>>,
    report: RefCell<String>
}

impl Master {

    fn new() -> Master {
        Master {
            slaves: RefCell::new(None),
            report: RefCell::new(String::new())
        }
    }

    fn declare(&self, a: &str) {
        let mut z = self.report.borrow_mut();
        z.push_str(a);
    }

    fn print_report(&self) {
        println!("{}", self.report.borrow());
    }

    fn set_slaves(&self, slaves: Vec<Rc<Slave>>) {
        self.slaves.replace(Some(slaves));
    }

}

struct Slave {
    name: String,
    master: Rc<Master>
}

impl Slave {
    
    fn new(name: String, master: Rc<Master>) -> Slave {
        Slave {
            name: name,
            master: master
        }
    }

    fn work(&self, i: u64) {
        for i in 0..i {
        }
        println!("Reporting...");
        self.master.declare(&format!("Reached {}\n", i.to_string()));
    }

}

pub fn run() {
    // Using a cycle reference to create a ManyToMany struct relationship
    // Note: The memory for all Master and Slave instances is freed for all if and only if all go out of scope.
    let master = Rc::new(Master::new());
    let slave1 = Rc::new(Slave {
        name: "bob".to_string(),
        master: master.clone()
    });
    let  slave2 = Rc::new(Slave {
            name: "charlie".to_string(),
            master: master.clone()
    });

    let v = vec!(
        slave1.clone(), 
        slave2.clone()
    );

    master.set_slaves(v);
    slave1.work(6);
    slave2.work(3);
    master.print_report();
}

