use std::io::{stdin, stdout, Write};

/// Simple Math
/// Write a program that prompts for two numbers.
/// Print the sum, difference, product, and quotient of those numbers
///
/// Constraints
/// - Values coming from users will be strings. Ensure that you convert these values to numbers before doing the math.
/// - Keep the inputs and outputs separate from the numerical conversions and other processing.
/// - Generate a single output statement with line breaks in the appropriate spots.
///
/// Challenges
/// - Revise the program to ensure that inputs are entered as
///   numeric values. Don’t allow the user to proceed if the
///   value entered is not numeric.
/// - Don’t allow the user to enter a negative number.
/// - Break the program into functions that do the computa-
///   tions.
fn main() {
    let num1: i32;
    let num2: i32;

    let mut input = String::new();

    loop {
        print!("What is the first number? ");
        stdout().flush().ok();
        stdin().read_line(&mut input).ok();

        match input.trim().parse::<i32>() {
            Ok(n) => {
                if n.is_negative() {
                    println!("Please enter a positive number.");
                } else {
                    break num1 = n
                }
            },
            Err(_) => println!("Please enter a number.")
        }
        input.clear();
    }

    input.clear();

    loop {
        print!("\nWhat is the second number? ");
        stdout().flush().ok();
        stdin().read_line(&mut input).ok();

        match input.trim().parse::<i32>() {
            Ok(n) => {
                if n.is_negative() {
                    println!("Please enter a positive number.");
                } else {
                    break num2 = n
                }
            }
            Err(_) => println!("Please enter a number.")
        }
        input.clear();
    }

    let msg = format!(r#"
         {} + {} = {}
         {} - {} = {}
         {} * {} = {}
         {} / {} = {}
    "#,
                      num1, num2, num1 + num2,
                      num1, num2, num1 - num2,
                      num1, num2, num1 * num2,
                      num1, num2, num1 / num2
    );

    println!("{}", msg);
}