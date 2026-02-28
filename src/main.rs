use rustyline::DefaultEditor;
use std::process::Command;

fn main() {
    let mut rl = DefaultEditor::new().unwrap();

    loop {
        let current_dir = std::env::current_dir().unwrap();
        let dir_name = current_dir.file_name().unwrap().to_string_lossy();
        let prompt = format!("{} $ ", dir_name);

        let readline = rl.readline(&prompt);

        match readline {
            Ok(line) => {
                let input = line.trim().to_string();

                if input.is_empty() {
                    continue;
                }

                rl.add_history_entry(&input).unwrap();

                if input == "exit" {
                    break;
                }

                let mut parts = input.split_whitespace();
                let command = parts.next();
                let args: Vec<&str> = parts.collect();

                match command {
                    Some("cd") => {
                        let dir = args.first().copied().unwrap_or("/");
                        if let Err(e) = std::env::set_current_dir(dir) {
                            println!("Error: {}", e);
                        }
                    }
                    Some(cmd) => {
                        let status = Command::new(cmd).args(args).status();
                        match status {
                            Ok(_) => {}
                            Err(e) => println!("Error: {}", e),
                        }
                    }
                    None => {}
                }
            }
            Err(_) => break,
        }
    }
}