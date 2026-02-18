mod args;
mod core;

use args::{Command, parser_args};

fn main() {
    pub fn init() {
        match parser_args() {
            Command::Init => match core::init::init_workspace() {
                Ok(_) => println!("Monux initialized successfully"),
                Err(e) => println!("Error {}", e),
            },
            Command::Unknown => {
                println!("Usage: monux init");
            }
        }
    }
}
