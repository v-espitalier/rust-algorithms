//! File System Utilities
//!
//! A collection of file and directory operations for Rust.
//! Includes functions for reading/writing text and binary files,
//! listing directory contents, and retrieving file information.
//!
//! Author: Vincent Espitalier
//! Date: June 2024

use std::fs::File;
use std::fs::{self, read_dir, Permissions};
use std::io::ErrorKind;
use std::io::{Read, Write};
use std::path::Path;
use std::time::SystemTime;

/// Tests if a file exists at the given path.
///
/// # Arguments
/// * `file_path` - Path to the file as a string.
///
/// # Returns
/// `true` if the file exists, `false` otherwise.
///
/// # Example
/// ```
/// assert_eq!(test_file_existence(&String::from("Cargo.toml")), true);
/// ```
pub fn test_file_existence(file_path: &String) -> bool {
    Path::new(file_path).exists()
}

/// Reads a text file and returns its content as a String.
///
/// # Arguments
/// * `file_path` - Path to the text file.
///
/// # Returns
/// The content of the file as a String.
///
/// # Panics
/// Panics if the file cannot be found or read.
///
/// # Example
/// ```
/// let content = read_text_file(&String::from("example.txt"));
/// ```
pub fn read_text_file(file_path: &String) -> String {
    // Open and read the text file
    // See: https://doc.rust-lang.org/book/ch12-02-reading-a-file.html
    // and: https://doc.rust-lang.org/std/fs/struct.File.html
    fs::read_to_string(file_path).expect("File not found")
}

/// Reads a text file and returns its content line by line as Vec<String>.
///
/// # Arguments
/// * `file_path` - Path to the text file.
/// * `separator_opt` - Optional line separator (defaults to "\n").
///
/// # Returns
/// A vector where each element is a line from the file.
///
/// # Example
/// ```
/// let lines = read_text_file_lines(&String::from("example.txt"), Some("\r\n"));
/// ```
pub fn read_text_file_lines(file_path: &String, separator_opt: Option<&str>) -> Vec<String> {
    // Read the text file
    let content = read_text_file(file_path);

    let default_separator = "\n";
    let separator = if let Some(elem) = separator_opt {
        elem
    } else {
        default_separator
    };
    content.split(separator).map(|s| s.to_string()).collect()
}

/// Writes content to a text file.
///
/// # Arguments
/// * `file_path` - Path to the output file.
/// * `content` - Content to write to the file.
///
/// # Panics
/// Panics if the file cannot be created or written to.
///
/// # Example
/// ```
/// write_text_file(&String::from("output.txt"), &String::from("Hello, world!"));
/// ```
pub fn write_text_file(file_path: &String, content: &String) {
    // See: https://doc.rust-lang.org/std/fs/struct.File.html
    let mut file = File::create(file_path).expect("Error: Could not create file.");
    file.write_all(content.as_bytes())
        .expect("Error: Could not write to file.");
    println!("File written: {}", file_path);
}

/// Writes a vector of strings to a text file, one string per line.
///
/// # Arguments
/// * `file_path` - Path to the output file.
/// * `content_vec` - Vector of strings to write (one per line).
///
/// # Example
/// ```
/// let lines = vec![String::from("Line 1"), String::from("Line 2")];
/// write_text_file_lines(&String::from("output.txt"), &lines);
/// ```
pub fn write_text_file_lines(file_path: &String, content_vec: &[String]) {
    write_text_file(file_path, &content_vec.join("\n"))
}

/// Reads a binary file and returns its content as a byte vector.
///
/// # Arguments
/// * `file_path` - Path to the binary file.
///
/// # Returns
/// A vector of bytes containing the file's content.
///
/// # Panics
/// Panics if the file cannot be found or read.
///
/// # Example
/// ```
/// let data = read_binary_file(&String::from("data.bin"));
/// ```
///
/// # Reference
/// Inspired by: https://www.reddit.com/r/rust/comments/dekpl5/how_to_read_binary_data_from_a_file_into_a_vecu8/
pub fn read_binary_file(file_path: &String) -> Vec<u8> {
    let mut file = File::open(file_path).expect("File not found");
    let size: usize = get_file_size(file_path) as usize;
    let mut buffer: Vec<u8> = vec![0; size];
    file.read_exact(&mut buffer).expect("Buffer overflow.");
    buffer
}

/// Writes binary data to a file.
///
/// # Arguments
/// * `file_path` - Path to the output file.
/// * `content` - Byte vector containing data to write.
///
/// # Panics
/// Panics if the file cannot be created or written to.
///
/// # Example
/// ```
/// let data = vec![0x48, 0x65, 0x6c, 0x6c, 0x6f]; // "Hello" in ASCII
/// write_binary_file(&String::from("output.bin"), &data);
/// ```
pub fn write_binary_file(file_path: &String, content: &Vec<u8>) {
    std::fs::write(file_path, content).expect("Error: Could not write binary file.");
}

/// Lists all elements in a directory.
///
/// # Arguments
/// * `dir_path` - Path to the directory.
///
/// # Returns
/// A vector of strings containing paths of directory elements.
///
/// # Panics
/// Panics if the directory cannot be found or read.
///
/// # Example
/// ```
/// let files = list_directory(&String::from("."));
/// ```
///
/// # Reference
/// See: https://stackoverflow.com/questions/66577339/collect-file-names-into-vecstr
pub fn list_directory(dir_path: &String) -> Vec<String> {
    let paths_res = read_dir(dir_path);
    match paths_res {
        Err(error) if error.kind() == ErrorKind::NotFound => {
            panic!("Directory not found");
        }
        Err(error) => {
            panic!("Unexpected error: {:?}", error)
        }
        Ok(result) => result
            .filter_map(|e| e.ok())
            .map(|e| e.path().to_string_lossy().into_owned())
            .collect::<Vec<_>>(),
    }
}

/// Enum representing different file types.
#[derive(Debug)]
pub enum FileType {
    RegularFile,
    Directory,
    SymbolicLink,
}

/// Struct containing essential file information.
#[derive(Debug)]
#[allow(dead_code)] // Actually used by debug print {:?}
pub struct FileInfo {
    pub file_type: FileType,
    pub permissions: Permissions,
    pub modified_date: SystemTime,
    pub size: u64,
}

/// Retrieves essential information about a file.
///
/// # Arguments
/// * `file_path` - Path to the file.
///
/// # Returns
/// A FileInfo struct containing:
/// - File type (regular file, directory, or symbolic link)
/// - Permissions
/// - Last modification date
/// - Size in bytes
///
/// # Panics
/// Panics if the file cannot be found or its metadata cannot be read.
///
/// # Example
/// ```
/// let info = get_file_info(&String::from("Cargo.toml"));
/// println!("{:?}", info);
/// ```
pub fn get_file_info(file_path: &String) -> FileInfo {
    let metadata = fs::metadata(file_path).expect("File not found.");

    let file_type = metadata.file_type();
    let mut file_type_opt: Option<FileType> = None;
    if file_type.is_file() {
        file_type_opt = Some(FileType::RegularFile);
    }
    if file_type.is_dir() {
        file_type_opt = Some(FileType::Directory);
    }
    if file_type.is_symlink() {
        file_type_opt = Some(FileType::SymbolicLink);
    }
    if file_type_opt.is_none() {
        panic!("Unrecognized file type: {}", file_path);
    }

    let permissions: Permissions = metadata.permissions();
    let modified_date: SystemTime = metadata.modified().expect("Error with metadata.modified()");
    let size: u64 = metadata.len();

    FileInfo {
        file_type: file_type_opt.unwrap(),
        permissions,
        modified_date,
        size,
    }
}

/// Gets the size of a file in bytes.
///
/// # Arguments
/// * `file_path` - Path to the file.
///
/// # Returns
/// The size of the file in bytes.
///
/// # Panics
/// Panics if the file cannot be found.
///
/// # Example
/// ```
/// let size = get_file_size(&String::from("Cargo.toml"));
/// ```
pub fn get_file_size(file_path: &String) -> u64 {
    fs::metadata(file_path).expect("File not found.").len()
}
