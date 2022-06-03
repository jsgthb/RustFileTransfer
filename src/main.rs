use dialoguer::{Select, theme::ColorfulTheme};

mod server;
mod client;

fn main() {
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
    match selection_options[selection] {
        "Server" => {
            server::start(); 
        }
        "Client" => {
            client::start();
        }
        _=> {
            println!("Closing application")
        }
    }
}