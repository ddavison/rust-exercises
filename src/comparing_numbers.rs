use std::cmp::Ordering;
use std::process::exit;
use inquire::Text;

/// Write a program that asks for three numbers. Check first to see that all numbers are different.
/// If they are not all different, then exit the program. Otherwise, display the largest number of
/// the three
///
/// Constraint
///     Write the algorithm manually. Don't use a built-in function for finding the largest num
fn main() {
    let mut numbers: Vec<i32> = Vec::new();

    loop {
        let mut input = Text::new("Enter a number:").with_help_message("q to quit").prompt().unwrap();
        input = input.trim().to_string();

        if input == "q" {
            break;
        } else {
            let num = match input.parse::<i32>() {
                Ok(num) => Some(num),
                Err(_) => None,
            };

            match num {
                Some(num) if numbers.contains(&num) => println!("Cannot use the same number twice."),
                Some(num) => numbers.push(num),
                None => println!("Invalid number")
            }
        }
    }

    if numbers.len() == 0 {
        println!("No numbers entered. Quitting.");
        exit(0);
    }

    // calculate the largest number
    let mut highest_number: &i32 = numbers.first().unwrap();
    for num in numbers.iter() {
        if num.cmp(highest_number) == Ordering::Greater { highest_number = num }
    }

    println!("The largest number is {}", highest_number);
}