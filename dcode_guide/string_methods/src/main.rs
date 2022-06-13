// String Methods

fn main() {
    // replace
    {
        let my_string = String::from("Rust is fantastic!");
        println!("After replace: {}", my_string.replace("fantastic", "great"));
    }

    // Lines
    // Split up your string into one iterator for each line in your string
    {
        let my_string = String::from("The weather is\nnice\noutside mate!");

        for line in my_string.lines() {
            println!("[ {} ]", line);
        }
    }

    // Split
    // Split with delimiter
    {
        let my_string = String::from("Leave a like if yo uenjoyed!");
        
        // a vector of string slices
        let tokens: Vec<&str> = my_string.split(" ").collect();

        println!("{}", my_string);
        println!("At indedx 2: {}", tokens[2]);

    }

    // Trim
    // get rid of whitespaces in this example
    {
        let my_string = String::from("    My name is Kenny     \n\r");
        println!("Before trim: {}", my_string);
        println!("After trim: {}", my_string.trim());
    }


    // Chars 
    // ??
    {
        let my_string = String::from("dcode on YouTube");
        println!("{}", my_string);
        // Get character at index
        match my_string.chars().nth(4) {
            // have to deal with both find and dont find that index (4)
            Some(c) => println!("Character at index 4: {}", c),
            None => println!("No chacter at index 4")
        };
    }

}
