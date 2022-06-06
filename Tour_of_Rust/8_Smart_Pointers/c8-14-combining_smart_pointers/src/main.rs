// this is way above my head
// and has been for a good while

use std::cell::RefCell;
use std::rc::Rc;

struct Pie {
    slices: u8,
}

impl Pie {
    fn eat_slice(&mut self, name: &str) {
        println!("{} took a slice!", name);
        self.slices -= 1;
    }
}

struct SeaCreature {
    name: String,
    pie: Rc<RefCell<Pie>>,
}

impl SeaCreature {
    fn eat(&self) {
        // use smart pointer to pie for a mutable borrow
        let mut p = self.pie.borrow_mut();
        // take a bite!
        p.eat_slice(&self.name);
    }
}

fn main() {
    let pie = Rc::new(RefCell::new(Pie { slices: 8 }));
    // ferris and sarah are giving clones of smart pointer to pie
    let ferris = SeaCreature {
        name: String::from("Ferris"),
        pie: pie.clone(),
    };
    let sarah = SeaCreature {
        name: String::from("Sarah"),
        pie: pie.clone(),
    };
    ferris.eat();
    sarah.eat();

    let p = pie.borrow();
    println!("{} slices left", p.slices);
}
