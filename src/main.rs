use dialoguer::{Select, theme::ColorfulTheme};

mod server;
mod client;

fn main() {
    // Server options
    // 0.0.0.0 address for external connections
    let address = "127.0.0.1";
    let port = 35000;
    // Prompt options
    let selection_options = &[
        "Server",
        "Client",
        "Exit",
    ];
    // Start program selection prompt
    let selection =  Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Select program")
        .default(0)
        .items(&selection_options[..])
        .interact()
        .unwrap();
    // Clear screen
    print!("\x1B[2J\x1B[1;1H");
    // Selected option
    match selection_options[selection] {
        "Server" => {
            server::start(address, port); 
        }
        "Client" => {
            client::start(address, port);
        }
        _=> {
            println!("Closing application")
        }
    }
}