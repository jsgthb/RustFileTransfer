use std::{fs, net::TcpListener};

struct File {
    name: String,
    path: String,
    size: u64
}

fn main() {
    // 0.0.0.0 address for external network accessibility
    let address = "127.0.0.1";
    let port = 35000;
    let mut file_array: Vec<File> = Vec::new();
    let mut counter = 0;
    println!("Checking files on server (/files/server)...");
    // Loop through all files in server directory
    for file in fs::read_dir("./files/server").unwrap() {
        // Store file data
        let file_name = file.as_ref().unwrap().file_name().into_string().unwrap();
        let file_path = file.as_ref().unwrap().path().display().to_string();
        let file_length = file.unwrap().metadata().unwrap().len();
        file_array.push(File { name: file_name.clone(), path: file_path, size: file_length.clone() });
        // Display file data
        println!("{}: '{}' {}", counter, file_name, pretty_print_filesize(file_length));
        counter += 1;
    }
    // End process if no files are located in server folder
    if counter == 0 {
        println!("No files found, shutting down");
        return;
    }
    // Start server
    let listener = TcpListener::bind(format!("{}:{}", address, port)).unwrap();
    println!("Server started on {}:{}", address, port);
    println!("Waiting for client...");
    for stream in listener.incoming() {
        match stream {
            // On successful connection
            Ok(stream) => {
                println!("Client connected: {}", stream.peer_addr().unwrap());
            }
            // On connection error
            Err(e) => {
                println!("Connection error: {}", e);
            }
        }
    }
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