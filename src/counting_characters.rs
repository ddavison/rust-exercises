use std::io::{stdin, stdout, Write};
use std::process::exit;

/// Counting the number of characters
/// Create a program that prompts for an input string
/// and displays output that shows the input string and the number of
/// characters the string contains.
///
/// If the user enters nothing, state that the user must enter
/// something into the program.
///
/// Implement this program using a graphical user interface
/// and update the character counter every time a key is
/// pressed. If your language doesn’t have a particularly
/// friendly GUI library, try doing this exercise with HTML
/// and JavaScript instead.
fn main() {
    let mut input = String::new();

    print!("What is the input string? ");
    stdout().flush().ok();
    stdin().read_line(&mut input).expect("invalid");

    if input.trim().is_empty() {
        println!("Enter something");
        exit(1);
    }

    count_characters(input);
}

fn count_characters(s: String) {
    let s = s.trim();
    println!("{} has {} characters.", s, s.len());
}