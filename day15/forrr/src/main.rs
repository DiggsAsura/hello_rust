fn main() {
    //let numbers = 30..51;
    // from 30 to 50, 51 is non-inclusive

    let animals = vec!["Rabbit", "Dog", "Cat"];

    for (index, a) in animals.iter().enumerate() {
        // make sure to use .iter(), else you drop the values of animals vector
        println!("The index {} is {}", index, a);
    }
}
