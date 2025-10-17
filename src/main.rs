use std::io;

fn main() {
    println!("Welcome to Smart Café Billing System!");
    println!("Enter the bill amount (in ₦):");

    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");

    let bill_amount: f64 = input.trim().parse().expect("Please enter a valid number");
    
    let discount = if bill_amount > 10000.0 {
        bill_amount * 0.15
    } else if bill_amount > 5000.0 {
        0.10 * bill_amount
    } else {
        0.0
    };
    
    let total_amount = bill_amount - discount;

    println!("Original Bill Amount: ₦{:.2}", bill_amount);
    println!("-----------------------------------");
    println!("Discount Applied: ₦{:.2}", discount);
    println!("Total Amount to Pay: ₦{:.2}", total_amount);
}