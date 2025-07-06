use colored::*;

/// Converts a vector of bytes into a string representation.
/// If logging is enabled, it prints the conversion process to the console.
/// # Arguments
/// * `vector` - A `Result` containing a vector of bytes or an `std::io::Error`.
/// * `log` - An optional reference to a boolean that indicates whether to log the conversion process.
/// # Returns
/// A `String` representation of the vector, formatted as `Result<Vec<u8>, std::io::Error>`.
pub(crate) fn vec_to_string(vector: Result<Vec<u8>, std::io::Error>, log: Option<&bool>) -> String{
    let string = format!("{:?}", vector);
    if let Some(true)=log {
        println!("{}","INFO: Converting vector to string".yellow());
        print!("{}","INFO: String from vector: ".yellow());
        println!("{}", string.yellow());
        }
    return string;
}