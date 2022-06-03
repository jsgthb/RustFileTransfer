use std::fs;

struct File {
    name: String,
    path: String,
    size: u64
}

fn main() {
    let mut fileArray: Vec<File> = Vec::new();
    let mut counter = 0;
    println!("Checking files on server (/files/server)...");
    // Loop through all files in server directory
    for file in fs::read_dir("./files/server").unwrap() {
        // Store file data
        let fileName = file.as_ref().unwrap().file_name().into_string().unwrap();
        let filePath = file.as_ref().unwrap().path().display().to_string();
        let fileLength = file.unwrap().metadata().unwrap().len();
        fileArray.push(File { name: fileName.clone(), path: filePath.clone(), size: fileLength.clone() });
        // Display file data
        println!("{}: {} {}", counter, fileName, fileLength);
        counter += 1;
    }
}
