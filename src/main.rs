use std::io::{self, Write};

fn main() {
    println!("Smart Weather Temperature Converter");
    println!("What do you want to convert to? \nCelsius or Fahrenheit (Pick 1 for Celsius and 2 for Fahrenheit.)");

    print!("Enter choice: ");
    io::stdout().flush().unwrap();
    let mut choice_input = String::new();
    io::stdin().read_line(&mut choice_input).expect("Failed to read choice");
    let choice = choice_input.trim().parse::<u32>().unwrap_or(0);

    if choice == 1 {
        println!("Celsius Converter \nInput the degree (enter value in Fahrenheit):");
        let mut degree_input = String::new();
        io::stdin().read_line(&mut degree_input).expect("Failed to read degree");
        let f = match degree_input.trim().parse::<f64>() {
            Ok(v) => v,
            Err(_) => {
                println!("Invalid number");
                return;
            }
        };
        let c = (f - 32.0) * 5.0 / 9.0;
        println!("Result: {:.2} °C", c);
    } else if choice == 2 {
        println!("Fahrenheit Converter \nInput the degree (enter value in Celsius):");
        let mut degree_input = String::new();
        io::stdin().read_line(&mut degree_input).expect("Failed to read degree");
        let c = match degree_input.trim().parse::<f64>() {
            Ok(v) => v,
            Err(_) => {
                println!("Invalid number");
                return;
            }
        };
        let f = c * 9.0 / 5.0 + 32.0;
        println!("Result: {:.2} °F", f);
    } else {
        println!("Invalid choice");
    }
} 