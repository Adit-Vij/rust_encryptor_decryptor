mod utilities;
mod cryptography;

use crate::utilities::str_input::str_input;
use crate::utilities::file_loader::load_file_bin;
use crate::utilities::file_loader::get_file_metadata;
use crate::utilities::file_loader::get_file_name;
use crate::utilities::vec_to_str::vec_to_string;
use crate::cryptography::hasher::sha256_hash;
use crate::utilities::decor_title::print_decor_title;

use colored::*;
fn main() {
    // Log flag, set as false to disable logging
    let  log = false;

    // Define chunk size
    let chunk_size = 1024;

    // Define a handler function to process each chunk
    // let handler = |chunk: &[u8]| -> std::io::Result<()> {
    //     // Process the chunk (for example, print its length)
    //     println!("Read {} bytes", chunk.len());
    //     Ok(())
    // };

    // Print a decorative header
    print_decor_title();

    // Get file path from user input
    let path = str_input("Enter the file path: ".to_string()).trim().to_string();
    println!("{}", "Loading file: ".blue());

    // Get file metadata
    if log == true {
        
        println!("{}", format!("{}",get_file_name(&path)).blue());
        println!("{}", format!("File path: {}", path).blue());
    }
    let metadata = get_file_metadata(&path);
    println!("{}", format!("File metadata: {:?}", metadata).blue());
    
    // Load the file in binary mode, processing it in chunks
    let _bin_content: Result<Vec<u8>, std::io::Error> = load_file_bin(path, chunk_size, Some(&log));
    
    // Unwrap the binary content once and store it
    let bin_content = _bin_content.unwrap();

    //hash the content
    if log == true {
        println!("{}", "Hashing bin contents...".yellow());
    }

    let hash = sha256_hash(&bin_content);
    println!("{}", format!("SHA-256 Hash: {}", hash).green());

    /* Print the binary data if needed
    println!("{}", format!("File loaded successfully: {:?}", bin_content).green());
    */

    let string_content = vec_to_string(Ok(bin_content), Some(&log));

    // Dummy input to keep the program running
    str_input("Press Enter to exit...".to_string());
    println!("{}", "Exiting...".blue());
    // Exit the program
    std::process::exit(0);
}