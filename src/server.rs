use std::io::{Write, self, BufRead};
use std::{fs};
use std::net::{TcpListener, TcpStream, Shutdown};
use serde_derive::{Serialize, Deserialize};

pub fn start(address: &str, port: i32) {
    let mut file_array: Vec<File> = Vec::new();
    let mut counter = 0;
    println!("Checking files on server (/files/server)...");
    // Loop through all files in server directory
    for file in fs::read_dir("./files/server").unwrap() {
        // Store file data
        let file_name = file.as_ref().unwrap().file_name().into_string().unwrap();
        let file_path = file.as_ref().unwrap().path().display().to_string();
        let file_length = file.unwrap().metadata().unwrap().len();
        // Check if filesize is below 10 MB
        if file_length > 10 * 1024 * 1024 {
            panic!("The maximum filesize is 10 MB")
        }
        file_array.push(File { name: file_name.clone(), path: file_path, size: file_length.clone() });
        // Display file data
        println!("{}: '{}' {}", counter, file_name, pretty_print_filesize(file_length));
        counter += 1;
    }
    // End process if no files are located in server folder
    if counter == 0 {
        panic!("No files found in files/server folder")
    }
    // Start server
    let listener = TcpListener::bind(format!("{}:{}", address, port)).unwrap();
    println!("Server started on {}:{}", address, port);
    println!("Waiting for client...");
    for stream in listener.incoming() {
        match stream {
            // On successful connection
            Ok(stream) => {
                handle_client(stream, &file_array);
            }
            // On connection error
            Err(e) => {
                println!("Connection error: {}", e);
            }
        }
    }
}

fn handle_client(mut stream: TcpStream, file_array: &Vec<File>) {
    println!("Client connected from {}", stream.local_addr().unwrap());
    let mut reader = io::BufReader::new( stream.try_clone().unwrap());
    // Send list of files to client
    let serialized_array = serde_json::to_string(&file_array).unwrap();
    stream.write_all(serialized_array.as_bytes()).unwrap();
    // Get client selection
    let received = reader.fill_buf().unwrap().to_vec();
    reader.consume(received.len());
    let selection = String::from_utf8(received).unwrap().parse::<usize>().unwrap();
    // Open selected file
    let opened_file = std::fs::read(file_array.get(selection).unwrap().path.clone()).unwrap();
    // Send file
    println!("Sending file {} to client {}", file_array.get(selection).unwrap().name.clone(), stream.local_addr().unwrap());
    stream.write_all(&opened_file).unwrap();
    println!("{} transfer completed", stream.local_addr().unwrap());
    stream.shutdown(Shutdown::Both).unwrap();
    println!("Client {} connection closed", stream.local_addr().unwrap());
}

#[derive(Serialize, Deserialize)]
pub struct File {
    pub name: String,
    pub path: String,
    pub size: u64
}

pub fn pretty_print_filesize(length: u64) -> String {
    if length < 1024 {
        return format!("{} Bytes", length);
    } else if length >= 1024 && length < 1024 * 1024 {
        let output_length: f64 = length as f64 / 1024 as f64;
        return format!("{:.2} kB", output_length);
    } else {
        let output_length: f64 = length as f64 / 1024 as f64 / 1024 as f64;
        return format!("{:.2} MB", output_length);
    }
}