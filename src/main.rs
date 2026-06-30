#![allow(unused_variables)]
use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        eprintln!("Usage: {} tokenize <filename>", args[0]);
        return;
    }

    let command = &args[1];
    let filename = &args[2];

    match command.as_str() {
        "tokenize" => {
            // You can use print statements as follows for debugging, they'll be visible when running tests.
            eprintln!("Logs from your program will appear here!");

            let file_contents = fs::read_to_string(filename).unwrap_or_else(|_| { // same as catching error
                eprintln!("Failed to read file {}", filename);
                String::new()
            });

            // TODO: Uncomment the code below to pass the first stage
            if !file_contents.is_empty() {
                eprintln!("this is file contents {}" ,file_contents)
                for ch in file_contents.chars(){
                    match ch {
                        '(' => println!("LEFT_PAREN ( null"),
                        ')' => println!("RIGHT_PAREN ) null"),
                        _ => {}
                    }
                }
                panic!("Scanner not implemented");
            } else {

                eprintln!("{}", file_contents);
                println!("EOF  null"); // Placeholder, replace this line when implementing the scanner
            }
        }
        _ => {
            eprintln!("Unknown command: {}", command);
        }
    }
}
