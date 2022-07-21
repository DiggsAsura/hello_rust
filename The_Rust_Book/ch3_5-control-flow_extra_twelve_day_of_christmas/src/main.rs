fn main() {
    let days = ["first", "second", "third", "fourth", "fifth"]; // 12 in the original
    let verses = ["A partridge in a pear tree", "Two Turtledoves", "Three French hens", "Four calling birds", "Five gold rings (five golden rings)"];

    for day in days {
        println!("On the {day} of Christmas, my true love sent to me");
        for verse in verses {
            println!("{verse}");
        }
        println!(""); 
    }

    println!("... (12 verses)");
}



fn verse_arrays() {
    let mut verse_one = [];
    let mut verse_two = [];
    let mut verse_three = [];

}


fn verse_strings() {
    let verse_one = "A partridge in a pear tree";
    let verse_two = "Two turtledoves";
    let verse_three = "Three French hnes";
}
