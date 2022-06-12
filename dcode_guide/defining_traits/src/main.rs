// How to define own Traits
// Traits are like certain set of rules or requirements that a object or struct 
// must have in order to have that name of the traits... uh?

struct Person {
    name: String,
    age: u8
}

trait HasVoiceBox {
    // Speak
    fn speak(&self);
    // Check if can speak (baby?)
    fn can_speak(&self) -> bool;
}

impl HasVoiceBox for Person {
    fn speak(&self) {
        println!("Hello, my name is {}", self.name);
    }

    fn can_speak(&self) -> bool {
        if self.age > 0 {
            return true;
        } return false;
    }
}



fn main() {
    let person = Person {
        name: String::from("Bob"),
        age: 41
    };
    //println!("Can {} speak? Answer: {}", person.name, person.can_speak());
    person.speak();
}
