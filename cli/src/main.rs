use clap::{App};
use std::env;
use std::process::Command;
fn main() {
    let matches = App::new("Securust CLI")
        .version("0.1.0")
        .author("tush-tr")
        .about("A CLI for Securust")
        .subcommand(App::new("start").about("Start the Securust UI server"))
        .get_matches();

    match matches.subcommand() {
        ("start", Some(_)) => {
            println!("Starting Securust UI server...");
            let ui_dir = "../ui";
            if let Err(err) = env::set_current_dir(ui_dir) {
                eprintln!("Error changing directory: {}", err);
                return;
            }
            let command = Command::new("npm")
                .args(&["start"])
                .spawn();
            match command {
                Ok(_) => {
                    println!("Securust UI server started successfully.");
                }
                Err(err) => {
                    eprintln!("Error starting Securust UI server: {}", err);
                }
            }

        }
        _ => println!("Invalid command"),
    }
}
