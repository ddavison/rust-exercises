use std::io::{stdin, stdout, Write};
use crate::Status::{InvalidPrice, InvalidQuantity, ReadyForCheckout};

#[derive(Debug)]
enum Status {
    InvalidPrice,
    InvalidQuantity,
    ReadyForCheckout,
}

struct Scan {
    price: f32,
    quantity: u32,
}

impl Scan {
    fn subtotal(&self) -> f32 {
        self.price * self.quantity as f32
    }

    fn tax(&self) -> f32 {
        self.subtotal() / 5.5
    }

    fn total(&self) -> f32 {
        self.subtotal() + self.tax()
    }
}

/// Create a simple self-checkout system. Prompt for the prices
/// and quantities of three items. Calculate the subtotal of the
/// items. Then calculate the tax using a tax rate of 5.5%. Print
/// out the line items with the quantity and total, and then print out the subtotal, tax amount, and total.
///
/// Keep the input, processing, and output parts of your
/// program separate. Collect the input, then do the math
/// operations and string building, and then print out the
/// output.
///
/// Be sure you explicitly convert input to numerical data
/// before doing any calculations.
///
/// Challenges
///
/// Revise the program to ensure that prices and quantities
/// are entered as numeric values. Don’t allow the user to
/// proceed if the value entered is not numeric.
///
/// Alter the program so that an indeterminate number of
/// items can be entered. The tax and total are computed
/// when there are no more items to be entered.
fn main() {
    println!("Welcome to the self-checkout!");
    println!("If you are done scanning, type 'checkout'");

    let mut scans: Vec<Scan> = Vec::new();

    loop {
        match scan() {
            Ok(scan) => {
                println!("\t {}x ${} = ${:.2} Scanned", &scan.quantity, &scan.price, scan.subtotal());
                scans.push(scan);
            },
            Err(status) => {
                match status {
                    ReadyForCheckout => break,
                    InvalidPrice => println!("Invalid Price"),
                    InvalidQuantity => println!("Invalid Quantity"),
                }
            }
        }
    }
    checkout(scans);
}

fn scan() -> Result<Scan, Status> {
    let mut input = String::new();
    let price: f32;
    let quantity: u32;

    print!("Enter the price of item (or type 'checkout'): $");
    stdout().flush().ok();
    stdin().read_line(&mut input).expect("Something went wrong");

    if input.trim().to_lowercase() == "checkout" {
        return Err(ReadyForCheckout)
    }

    match input.trim().parse::<f32>() {
        Ok(fl) => price = fl,
        Err(_) => return Err(InvalidPrice)
    }

    input.clear();
    print!("Enter the quantity of item: ");
    stdout().flush().ok();
    stdin().read_line(&mut input).expect("Something went wrong");

    match input.trim().parse::<u32>() {
        Ok(q) => quantity = q,
        Err(_) => return Err(InvalidQuantity),
    }

    Ok(Scan { price, quantity })
}

fn checkout(scans: Vec<Scan>) { // take ownership of the scans since we're no longer going to use it.
    println!("=====================================");
    println!("Number of scans: {}", scans.len());
    println!("---------------------");
    println!("Subtotal: {:.2}", scans.iter().map(|scan| scan.subtotal()).sum::<f32>());
    println!("Total tax: {:.2}", scans.iter().map(|scan| scan.tax()).sum::<f32>());
    println!("---------------------");
    println!("Total: {:.2}", scans.iter().map(|scan| scan.total()).sum::<f32>());
    println!("=====================================");
    println!("Thank you for shopping at our self-checkout!")
}