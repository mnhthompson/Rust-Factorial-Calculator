use std::io;

fn main() {

    println!("Factorial  Calculator");
    println!(); 

            // Ask the user if they want a definition of a factorial
            print!("Would you like a definition of factorials? (y/n): ");
            io::Write::flush(&mut io::stdout()).expect("Failed to flush stdout");
    
            let mut show_definition = String::new();
            io::stdin().read_line(&mut show_definition).expect("Failed to read line");
    
            if show_definition.trim().to_lowercase() == "y" {
                print_factorial_definition();
            }

    loop {
    //Define a variable 'number' and assign a value to it
    print!("Type a whole number between 1 - 32: ");

    // Ensure text appears before input
    io::Write::flush(&mut io::stdout()).expect("Failed to flush stdout"); 

    let mut line = String::new();

    io::stdin().read_line(&mut line).expect("Failed to read line");

        let number: u128 = match line.trim().parse() {
            Ok(n) => n,
         // error protection
            Err(_) => {
                println!("⚠️ Invalid input! ⚠️ Please enter a valid number.");
                println!(); 
                continue; // Restart the loop
            }
        };

        let numberexit = number.to_string();

        // error protection
        if number > 32 {
            // Restart the loop
            println!("⚠️Number too large! ⚠️ Please enter a whole number between 1 and 32.");
            println!(); 
            continue;
        }

        // quit out the app
        if numberexit.to_lowercase() == "0" {
            println!("Exiting");
            break;
        }

    // Ask if the user wants to see the factorial table
    print!("Do you want to see the factorial table from 1 to {}? (y/n) ", number);

       // Ensure text appears before input
       io::Write::flush(&mut io::stdout()).expect("Failed to flush stdout");  


    let mut show_table = String::new();
    io::stdin().read_line(&mut show_table).expect("Failed to read line");

    if show_table.trim().to_lowercase() == "y" {
        // Display a factorial table for numbers 1 through the input number
        println!("Factorial table from 1 to {}:", number);
        println!(); 
        for i in 1..=number {
            let factorial_i = factorial(i);
            println!("Factorial of {} is: {}", i, factorial_i);
            println!(); 
        }
    }
    else {
        // Call the 'factorial' function with 'number' as an argument
        // Store the result in a variable named 'factorial_result'
        let factorial_result = factorial(number);

        // Print the result
        println!("\nFactorial of {} is: {}\n", number, factorial_result);}

    // Print the result
     
    println!("If your done Type 0 to Exit ");
    println!(); 
}
}

// Define a function named 'factorial' that takes a parameter 'n' of type u128 and returns a u128
fn factorial(n: u128) -> u128 {
    // Initialize the result variable with 1
    let mut result = 1;

    // Start a for loop to calculate the factorial
    for i in 1..=n {
        // Multiply the result by the current value of 'i'
        result *= i;
    }
    // Return the calculated factorial
    result
} 

// Definition of factorial
fn print_factorial_definition() {
    println!("Definition of a Factorial:");
    println!("A factorial, denoted by 'n!', is the product of all positive integers less than or equal to a given number 'n'.");
    println!("\nFor example:");
    println!("5! = 5 × 4 × 3 × 2 × 1 = 120");
    println!("Factorial of 0 is defined as 1: 0! = 1");
    println!();
}