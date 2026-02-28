use std::io::{self, Write};
use std::process::Command;

fn main() {
    loop {
        let current_dir = std::env::current_dir().unwrap();
        let dir_name = current_dir.file_name().unwrap().to_string_lossy();
        print!("{}$ ", dir_name);
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();

        let input = input.trim();

        if input == "exit" {
            break;
        }

        let mut parts = input.split_whitespace();
        let command = parts.next();
        let args: Vec<&str> = parts.collect();

        match command {
            Some("cd") => {
                let dir = args.iter().peekable().peek().map_or("/", |x| *x);
                if let Err(e) = std::env::set_current_dir(dir){
                    println!("Error: {}", e)
                }
            }

            Some(cmd) => {
                let status = Command::new(cmd)
                    .args(args)
                    .status();

                match status {
                    Ok(_) => {},
                    Err(e) => println!("Error: {}", e),
                }
            }
            None => {}
            
        }
    }
} 