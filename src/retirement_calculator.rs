use chrono::{Datelike, Local};
use read_input::prelude::*;

/// Retirement Calculator
///
/// Create a program that determines how many years you have
/// left until retirement and the year you can retire. It should
/// prompt for your current age and the age you want to retire
/// and display the output as shown in the example that follows.
fn main() {
    let current_year = Local::now().year();
    let current_age: i32 = input().msg("What is your current age? ")
        .inside_err(1.., "Invalid Age")
        .err("That doesn't appear to be a number")
        .get();
    let retirement_age: i32 = input().msg("At what age would you like to retire? ").get();
    let retirement_year = current_year + retirement_age;
    let years_left = retirement_age - current_age;

    let msg = format!(r#"
        You have {} years left until you can retire.
        It's {}, so you can retire in the year {}
    "#, years_left, current_year, retirement_year);

    println!("{}", msg);
}