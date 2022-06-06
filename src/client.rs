use std::net::{TcpStream};
use std::io;
use serde_derive::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
struct File {
    name: String,
    path: String,
    size: u64
}

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

fn pretty_print_filesize(length: u64) -> String {
    if length < 1024 {
        return format!("{} Bytes", length);
    } else if length >= 1024 && length < 1024 * 1024 {
        let output_length: f64 = length as f64 / 1024 as f64;
        return format!("{} kB", output_length);
    } else {
        let output_length: f64 = length as f64 / 1024 as f64 / 1024 as f64;
        return format!("{:.2} MB", output_length);
    }
}