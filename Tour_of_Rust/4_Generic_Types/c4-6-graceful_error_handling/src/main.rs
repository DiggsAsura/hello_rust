fn do_something_that_might_fail(i: i32) -> Result<f32, String> { 
    if i == 42 {
        Ok(13.0)
    } else {
        Err(String::from("this is not the right number"))
    }
}

fn main() -> Result<(), String> {
    // Look at how much code we saved!
    let v = do_something_that_might_fail(42)?;
    println!("found {}", v);
    Ok(())
}