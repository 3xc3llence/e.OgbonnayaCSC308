use std::io;

fn main() {
    println!("Welcome to Smart Energy Company (SEC) Billing Calculator!");
    println!("Enter your energy consumption (in kWh):");

    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");

    let usage: f64 = match input.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Invalid input. Please enter a valid number.");
            return;
        }
    };

    if usage < 0.0 {
        println!("Usage cannot be negative. Please enter a valid number.");
        return;
    }

    let mut bill = 0.0;

    if usage > 100.0 {
        bill += 100.0 * 20.0;
        let remaining = usage - 100.0;
        
        if remaining > 100.0 {
            bill += 100.0 * 25.0;
            let excess = remaining - 100.0;
            bill += excess * 30.0; 
        } else {
            bill += remaining * 25.0;
        }
    } else {
        bill += usage * 20.0;
    }

    println!("\n--- Billing Summary ---");
    println!("Energy Usage: {:.2} kWh", usage);
    println!("Total Bill: ₦{:.2}", bill);
}