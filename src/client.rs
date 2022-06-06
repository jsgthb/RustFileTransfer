use std::net::{TcpStream};
use std::io;

use crate::server::{File, pretty_print_filesize};

pub fn start(address: &str, port: i32) {
    match TcpStream::connect(format!("{}:{}", address, port)) {
        Ok(mut stream) => {
            println!("Successfully connected to server on port {}", port);
            let mut reader = io::BufReader::new(&mut stream);
            // Read current current data in the TcpStream
            let file_array: Vec<File> = serde_cbor::from_reader(reader).unwrap();
            println!("Select a file to download:");
            for (i, file) in file_array.iter().enumerate() {
                println!("{}: '{}' {}", i, file.name, pretty_print_filesize(file.size));
            }
        },
        Err(e) => {
            println!("Failed to connect: {}", e);
        }
    }
    println!("Terminated.");
}