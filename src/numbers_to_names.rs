use std::collections::HashMap;
use inquire::CustomType;
use inquire::validator::Validation;

/// Numbers to Names
///
/// Write a program that converts a number from 1 to 12 to the corresponding month.
/// Prompt for a number and display the corresponding calendar month, with 1 being January and
/// 12 being December. For any valud outside that range, display an appropriate error message.
fn main() {
    let mut months: HashMap<u32, &str> = HashMap::new();
    months.insert(1, "January");
    months.insert(2, "February");
    months.insert(3, "March");
    months.insert(4, "April");
    months.insert(5, "May");
    months.insert(6, "June");
    months.insert(7, "July");
    months.insert(8, "August");
    months.insert(9, "September");
    months.insert(10, "October");
    months.insert(11, "November");
    months.insert(12, "December");

    let month_number = CustomType::<u32>::new("Please enter the number of the month:")
        .with_help_message("Enter a number corresponding with a month (1-12)")
        .with_validator(|val: &u32| {
            match *val {
                1..=12 => Ok(Validation::Valid),
                _ => Ok(Validation::Invalid("Enter a number between 1 and 12".into()))
            }
        }).prompt().unwrap();

    println!("The name of the month is {}.", months.get(&month_number).unwrap());
}