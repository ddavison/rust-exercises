use std::io::{stdin, stdout, Write};

fn main() {
    // create a simple mad-lib program that prompts for a noun, a verb, an adverb, and an adjective and injects those into a story that you create.

    let noun = prompt("Enter a noun");
    let verb = prompt("Enter a verb");
    let adjective = prompt("Enter an adjective");
    let adverb = prompt("Enter an adverb");

    println!(
        "One day, the {noun} {adverb} {verb} their {adjective} shoes"
    )
}

fn prompt(p: &str) -> String {
    let mut s = String::new();

    print!("{}: ", p);
    stdout().flush().ok();
    stdin().read_line(&mut s).expect("Invalid");

    s.trim().to_owned()
}