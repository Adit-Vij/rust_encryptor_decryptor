use std::io::{self, Write};

/// Function to read a string input from the user.
/// # Arguments
/// * `prompt` - A string that will be displayed as a prompt to the user.
/// # Returns
/// * `String` - The input string provided by the user, trimmed of any trailing whitespace.
pub(crate) fn str_input(prompt: String)-> String{
    let mut input = String::new();
    print!("{} ", prompt);
    io::stdout().flush().expect("Failed to flush stdout");
    io::stdin().read_line(&mut input).expect("Failed to read line");
    input.trim_end().to_string();
    return input;
}