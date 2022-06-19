use std::io::{stdin, stdout, Write};

fn read(input: &mut String) {
    stdout().flush()
        .expect("failed to flush");
    stdin().read_line(input)
        .expect("failed to read");
}

fn main() {
    println!("Welcome to easy calc");
    println!("-----------------------------");

    let mut num1 = String::new();
    let mut num2 = String::new();
    let mut operator = String::new();
    
    loop {
        print!("What is the first number?: ");
        read(&mut num1);

        print!("What is the second number?: ");
        read(&mut num2);

        print!("What operation would you like to do? [+-*/]: ");
        read(&mut operator);

//    println!("{} {} {}", num1, num2, operator)

        let num1: f32 = num1.trim().parse().unwrap();
        let num2: f32 = num2.trim().parse().unwrap();
        let operator: char = operator.trim().chars().next().unwrap();

        let operatiors = String::from("+-*/");

        if !operatiors.contains(operator) {
            println!("Unknown operator");
            continue;
    }

        let result = match operator {
            '+' => num1 + num2,
            '-' => num1 - num2,
            '*' => num1 * num2,
            '/' => num1 / num2,
            _ => panic!("Error in operator")
        };

        println!("The result of {} {} {} = {}", num1, operator, num2, result);
    }

}
