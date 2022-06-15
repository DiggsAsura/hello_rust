// executing commands from your rust application
// run a python script and print out the result

use std::process::Command;

fn main() {
    // py kenny.py
    let mut cmd = Command::new("python3");
    cmd.arg("kenny.py");

    // Execute the command
    match cmd.output() {
        Ok(o) => {
            // Unsafe because it does not check if the vector of bytes is 
            // a valid UTF8 string
            unsafe {
                println!("Output: {}", String::from_utf8_unchecked(o.stdout));
            }
        },
        Err(e) => {
            println!("The error: {}", e);
        }
    }
}
