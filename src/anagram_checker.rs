use std::process::exit;
use inquire::Text;

/// Anagram Checker
///
/// Create a program that compares two strings and determines if the two strings are anagrams.
/// The program should prompt for both input strings and display output.
///
/// Constraints
/// - Implement the program by using a function called `is_anagram`, which takes two words as its
///   arguments and returns `true` or `false`. Invoke this function from your main program.
/// - Check that both words are the same length
///
/// Challenge
/// - Complete this program without using built-in language features. Use selection or repetition logic
///   instead and develop your own algorithm.
fn main() {
    println!("Enter two strings and I'll tell you if they are anagrams.");

    let first_word: String = Text::new("Enter the first word:").prompt().unwrap();
    let second_word: String = Text::new("Enter the second word:").prompt().unwrap();

    if first_word.len() != second_word.len() {
        println!("Both words must be of same length");
        exit(1);
    }

    match is_anagram(&first_word, &second_word) {
        true => println!("\"{}\" and \"{}\" are anagrams.", first_word, second_word),
        false => println!("\"{}\" and \"{}\" are not anagrams.", first_word, second_word)
    }
}

fn is_anagram(first_word: &String, second_word: &String) -> bool {
    let mut a: Vec<&char> = Vec::new();



    true
}
