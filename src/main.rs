use rustyline::DefaultEditor;
use std::process::Command;
use std::process::Stdio;

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
                

                if input.contains('>') {
                    let parts: Vec<&str> = input.split('>').collect();
                    let cmd_part = parts[0].trim();
                    let file_path = parts[1].trim();

                    let mut cmd_parts = cmd_part.split_whitespace();
                    let command = cmd_parts.next().unwrap();
                    let args: Vec<&str> = cmd_parts.collect();

                    let file = std::fs::File::create(file_path).unwrap();

                    Command::new(command)
                        .args(args)
                        .stdout(Stdio::from(file))
                        .status()
                        .unwrap();
                }
                else if input.contains('|') {
                    let commands: Vec<&str> = input.split('|').map(|cmd| cmd.trim()).collect();
                    
                    let mut prev_output = None;
                    let mut last_child = None;  // add this before the loop

                    for cmd_str in &commands {
                        let mut parts = cmd_str.split_whitespace();
                        let command = parts.next().unwrap();
                        let args: Vec<&str> = parts.collect();

                        let is_last = cmd_str == commands.last().unwrap();

                        let stdout = if is_last {
                            Stdio::inherit()
                        } else {
                            Stdio::piped()
                        };

                        let stdin = match prev_output {
                            None => Stdio::inherit(),
                            Some(output) => Stdio::from(output),
                        };

                        let mut child = Command::new(command)
                            .args(&args)
                            .stdin(stdin)
                            .stdout(stdout)
                            .spawn()
                            .unwrap();

                        prev_output = child.stdout.take();
                        last_child = Some(child);  // track the child
                    }

                    // after the loop, wait for the last one
                    if let Some(mut child) = last_child {
                        child.wait().unwrap();
                    }
                } else {
                    
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
            }
            Err(_) => break,
        }
    }
}