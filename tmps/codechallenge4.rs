fn main() {
    let first = "Summer mode has hit";
    let second = "Game mode is on";
    let third = "It's super hard to keep motivated for programming";
    let fourth = "But I'll get back to it";
    let fifth = "..oh and RL Is a fucking mess RN, in and out hospital for GF. It's rough.";

    let my_sentences = vec![first, second, third, fourth, fifth];

    for sentence in my_sentences {
        println!("{}", sentence);
    }
}
