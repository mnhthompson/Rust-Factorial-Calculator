use std::io;


fn main() {

    //Define a variable 'number' and assign a value to it

    println!("Type a number : ");

    let mut line = String::new();

    io::stdin()
        .read_line(&mut line)
        .expect("Failed to read line");

    let number: u64 = line
        .trim()
        .parse()
        .expect("Wanted a number");

    // Call the 'factorial' function with 'number' as an argument
    // Store the result in a variable named 'factorial_result'
    let factorial_result = factorial(number);

    // Print the result
    println!("Factorial of {} is: {}", number, factorial_result);
}

// Define a function named 'factorial' that takes a parameter 'n' of type u64 and returns a u64
fn factorial(n: u64) -> u64 {
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