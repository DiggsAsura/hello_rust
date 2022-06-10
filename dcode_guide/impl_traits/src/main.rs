// Trait is an interface (in java..) 
// Something a object, class, or struct can do

struct Person {
    name: String,
    age: u8
}

// implement the ToString Trait which is defind in the standard library
// the ToString trait contains one method
impl ToString for Person {
    fn to_string(&self) -> String { // new function, return a string
        return format!("My name is {} and I am {} years old.", self.name, self.age);
    }
}

fn main() {
    let ken = Person { name: String::from("Kenneth"), age: 38 };

    println!("{}", ken.to_string());
}
