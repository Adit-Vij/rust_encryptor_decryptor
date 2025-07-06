use std::fs::File;
use std::io::{self, BufReader, Read};
use std::path::Path;
use colored::*;

/// Loads a file in chunks and processes each chunk with the provided handler function.
/// The file is read in binary mode, and the handler function is called with each chunk of data.
/// # Arguments
/// * `path` - The path to the file to be loaded.
/// * `chunk_size` - The size of each chunk to read from the file.
/// * `handler` - A closure that takes a byte slice and returns a Result.
/// * `log` - An optional boolean reference to control logging behavior.
/// If `log` is `Some(true)`, it will log when EOF is reached.
/// # Returns
/// * `io::Result<Vec<u8>>` - Returns Ok if the file was read successfully, or an error if there was an issue reading the file.
pub(crate) fn load_file_bin(path: String, chunk_size: usize, log: Option<&bool>) -> io::Result<Vec<u8>> {
    let file = File::open(path)?;
    let mut reader = BufReader::new(file);
    let mut buffer = vec![0u8; chunk_size];
    let mut data = Vec::new();

    loop {
        let bytes_read = reader.read(&mut buffer)?;
        if bytes_read == 0 {
            if let Some(true) = log {
                println!("{}","INFO: EOF Reached".yellow());
            }
            break;
        }
        data.extend_from_slice(&buffer[..bytes_read]);
    }
    Ok(data)
}

/// Retrieves the metadata of a file at the specified path.
/// # Arguments
/// * `path` - The path to the file for which metadata is to be retrieved.
/// # Returns
/// * `io::Result<std::fs::Metadata>` - Returns Ok with the file metadata if successful, or an error if there was an issue accessing the file.
pub(crate) fn get_file_metadata(path: &str) -> io::Result<std::fs::Metadata>{
    return std::fs::metadata(path);
}

/// Extracts the file name from a given file path.
/// # Arguments
/// * `path` - The file path from which to extract the file name.
/// # Returns
/// * `String` - Returns the file name as a String. If the file name cannot be determined, it returns "Unknown File".
pub(crate) fn get_file_name(path: &str) -> String{
    Path::new(path)
        .file_name()
        .and_then(|name| name.to_str())
        .unwrap_or("Unknow File")
        .to_string()
}