use std::fmt::format;
use std::net::{TcpStream};
use std::io::{self, BufRead, Write};

use dialoguer::Select;
use dialoguer::theme::ColorfulTheme;

use crate::server::{File, pretty_print_filesize};

pub fn start(address: &str, port: i32) {
    match TcpStream::connect(format!("{}:{}", address, port)) {
        Ok(mut stream) => {
            println!("Successfully connected to server on port {}", port);
            let mut reader = io::BufReader::new(&mut stream);
            // Read current current data in the TcpStream
            let received = reader.fill_buf().unwrap().to_vec();
            reader.consume(received.len().clone());
            // Convert data to array of file structs
            let file_array: Vec<File> = serde_json::from_str(&String::from_utf8(received).unwrap()).unwrap();
            // Store data for selection
            let mut selection_options: Vec<String> = vec![];
            for (i, file) in file_array.iter().enumerate() {
                let selection = format!("{} ({})", file.name, pretty_print_filesize(file.size));
                selection_options.insert(i, selection);
            }
            // Select file to transfer
            let selection =  Select::with_theme(&ColorfulTheme::default())
                .with_prompt("Select file to download")
                .default(0)
                .items(&selection_options[..])
                .interact()
                .unwrap();
        },
        Err(e) => {
            println!("Failed to connect: {}", e);
        }
    }
    println!("Terminated.");
}