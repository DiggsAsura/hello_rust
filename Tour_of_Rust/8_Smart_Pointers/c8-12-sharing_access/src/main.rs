use std::cell::RefCell;

struct Pie {
   slices: u8 
}

impl Pie {
    fn eat(&mut self) {
        println!("tastes better on the heap!");
        self.slices -= 1;
    }
}

fn main() {
    // RefCell validates memory safety at runtime
    // notice: pie_cell is not mut!
    let pie_cell = RefCell::new(Pie{slices:8});

    {
        // but we can borrow mutable references!
        let mut mut_ref_pie = pie_cell.borrow_mut();
        mut_ref_pie.eat();
        mut_ref_pie.eat();

        // mut_ref_pie is dropped at end of scope
    }

    // now we can borrow immutably once our mutable reference drops
    let ref_pie = pie_cell.borrow();
    println!("{} slices lift", ref_pie.slices);
}
