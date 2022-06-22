use std::fs;
use std::net::{TcpStream};
use std::io::{self, BufRead, Write};

use dialoguer::Select;
use dialoguer::theme::ColorfulTheme;

use crate::server::{File, pretty_print_filesize};

pub fn start(address: &str, port: i32) {
    match TcpStream::connect(format!("{}:{}", address, port)) {
        Ok(mut stream) => {
            println!("Successfully connected to server on port {}", port);
            let mut reader = io::BufReader::with_capacity(10 * 1024 * 1204, stream.try_clone().unwrap());
            // Read current current data in the TcpStream
            let received = reader.fill_buf().unwrap().to_vec();
            reader.consume(received.len().clone());
            // Convert data to array of file structs
            let file_array: Vec<File> = serde_json::from_str(&String::from_utf8(received).unwrap()).unwrap();
            // Store data for selection
            let mut selection_options: Vec<String> = vec![];
            let mut file_names: Vec<String> = vec![];
            for (i, file) in file_array.iter().enumerate() {
                let selection = format!("{} ({})", file.name, pretty_print_filesize(file.size));
                selection_options.insert(i, selection);
                file_names.insert(i, file.name.clone())
            }
            // Select file to transfer
            let selection =  Select::with_theme(&ColorfulTheme::default())
                .with_prompt("Select file to download")
                .default(0)
                .items(&selection_options[..])
                .interact()
                .unwrap();
            stream.write_all(selection.to_string().as_bytes()).unwrap();
            // Receive file
            println!("Downloading file... ");
            let received_file = reader.fill_buf().unwrap().to_vec();
            reader.consume(received_file.len().clone());
            // Write file to disk
            fs::write(format!("./files/client/{}", file_names.get(selection).unwrap()), received_file).unwrap();
            println!("File successfully downloaded");
        },
        Err(e) => {
            println!("Failed to connect: {}", e);
        }
    }
    println!("Terminated.");
}