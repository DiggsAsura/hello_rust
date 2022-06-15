// does not work in 2022. need async/await

extern crate reqwest;

fn main() {
    let response_text = reqwest::get("http://youtube.local/hello")
        .expect("Couldn't make request")
        .text().expect("Could not read response text!");
    
    println!("Response Text: {}", response_text);
}
        
