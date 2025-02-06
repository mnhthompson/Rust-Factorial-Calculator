use std::io;

fn main() {

    loop {
    //Define a variable 'number' and assign a value to it
    print!("Type a whole number between 1 - 32: ");

    // Ensure text appears before input
    io::Write::flush(&mut io::stdout()).expect("Failed to flush stdout"); 

    let mut line = String::new();

    io::stdin()
        .read_line(&mut line)
        .expect("Failed to read line");

        let number: u128 = match line.trim().parse() {
            Ok(n) => n,
         // error protection
            Err(_) => {
                println!("⚠️ Invalid input! ⚠️ Please enter a valid number.");
                continue; // Restart the loop
            }
        };

        let numberexit = number.to_string();

        // error protection
        if number > 32 {
            // Restart the loop
            println!("⚠️Number too large! ⚠️ Please enter a whole number between 1 and 32.");
            continue;
        }

        // quit out the app
        if numberexit.to_lowercase() == "0" {
            println!("Exiting");
            break;
        }

    // Call the 'factorial' function with 'number' as an argument
    // Store the result in a variable named 'factorial_result'
    let factorial_result = factorial(number);

    // Print the result
    println!("Factorial of {} is: {}", number, factorial_result);

    println!("Type 0 to Exit ");

}}

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