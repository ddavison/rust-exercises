use inquire::CustomType;
use inquire::validator::Validation;

/// Pizza Party!! Pizza. Party. Pizza. Party.
///
/// Write a program to evenly divide pizzas.
/// Prompt for the number of people, the number of pizzas, and the number of slices per pizza.
/// Ensure that the number of pieces comes out even.
/// Display the number of pieces of pizza each person should get. If there are leftovers,
/// show the number of leftover pieces.
fn main() {
    let number_of_people = CustomType::<u32>::new("How many people?")
        .with_validator(|val: &u32| {
            match *val {
                0 => Ok(Validation::Invalid("Number must be greater than 0".into())),
                _ => Ok(Validation::Valid)
            }
        }).prompt().unwrap();

    let number_of_pizzas = CustomType::<u32>::new("How many pizzas?")
        .with_validator(|val: &u32| {
            match *val {
                0 => Ok(Validation::Invalid("Number must be greater than 0".into())),
                _ => Ok(Validation::Valid)
            }
        }).prompt().unwrap();

    let number_of_slices_per = CustomType::<u32>::new("How many slices per pizza?")
        .with_help_message("Must be an even number")
        .with_validator(|val: &u32| {
            match *val % 2 {
                v if v == 0 => Ok(Validation::Valid),
                _ => Ok(Validation::Invalid("Number of slices must be even".into()))
            }
        }).prompt().unwrap();

    let number_of_pieces_per_person = (number_of_pizzas * number_of_slices_per) / number_of_people;
    let number_of_pieces_leftover = (number_of_pizzas * number_of_slices_per) % number_of_people;

    println!("{} people with {} pizzas", number_of_people, number_of_pizzas);
    println!("Each person gets {} pieces of pizza.", number_of_pieces_per_person);
    println!("There are {} leftover pieces.", number_of_pieces_leftover);
}