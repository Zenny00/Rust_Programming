// Name: Joshua Comfort
// Date: 3/17/2023
// Done as part of learning the Rust programming language
// Take user input and reverse it

// read_line and flush functions
use std::io;
use std::io::Write;

// Reverse the passed string and return it
fn reverse(user_input: &String) -> String
{
    // Reverse the user's String and return it
    let reversed: String = user_input.chars().rev().collect();
    return reversed;
}

fn main()
{
    print!("Please enter a string to reverse: ");
    
    // print! may not always print if buffer is queued, use flush to force
    io::stdout().flush().unwrap();

    // Get user input from the console
    let mut user_string = String::new();
    io::stdin().read_line(&mut user_string).expect("Error reading input."); // Use mut to allow stdin() to modify the contents of user_string 

    // Remove newline
    user_string.truncate(user_string.len() - 1);

    // Print the result to the console
    print!("{} reversed is: {}\n", user_string, reverse(&user_string));
}
