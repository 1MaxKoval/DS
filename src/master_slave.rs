use std::cell::RefCell;

struct Master <'a> {
    slaves: RefCell<Option<Vec<Box<Slave<'a>>>>>,
    report: RefCell<String>
}

impl <'a> Master<'a> {

    fn new() -> Master<'a> {
        Master {
            slaves: RefCell::new(None),
            report: RefCell::new(String::new())
        }
    }

    fn declare(&self, a: &str) {
        let mut z = self.report.borrow_mut();
        z.push_str(a);
    }

    fn set_slaves(&self, slaves: Vec<Box<Slave<'a>>>) {
        self.slaves.replace(Some(slaves));
    }

}

struct Slave <'a> {
    name: String,
    master: &'a Master<'a>
}

impl <'a> Slave<'a> {
    
    fn new(name: String, master: &'a Master<'a>) -> Slave<'a> {
        Slave {
            name: name,
            master: master
        }
    }

    fn work(&self, i: u64) {
        for i in 0..i {
        }
        println!("Reporting...");
        self.master.declare(&format!("Reached {}", i.to_string()));
    }

}

pub fn run() {
    let master = Master::new();
    let v = vec!(
        Box::new(Slave {
            name: "bob".to_string(),
            master: &master
        }),
        Box::new(Slave {
            name: "charlie".to_string(),
            master: &master
        })
    );

    master.set_slaves(v);


}

