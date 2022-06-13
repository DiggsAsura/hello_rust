// put regex in cargo.toml

extern crate regex;
use regex::Regex;

fn main() {
    // r"" means raw 
    // {5}
//    let re = Regex::new(r"\w{5}").unwrap();
    let re = Regex::new(r"(\w{5})").unwrap();
    let text = "kenny";

    println!("Found match? {}", re.is_match(text));

    match re.captures(text) {
        Some(caps) => println!("Found match: {}", &caps[0]),
        //Some(caps) => println!("Found match: {}", caps.get(0).unwrap().as_str()),
        None => println!("Could not find match...")
    }
}
