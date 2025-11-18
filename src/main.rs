use std::io::{
    self,
    Write,
};

fn main() {
    loop {
        print!("$ ");

        io::stdout().flush().unwrap();

        let mut input = String::new();

        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        input = input.trim().to_string(); // remove newline

        if input.is_empty() {
            continue;
        }

        let cmd_line: Vec<&str> = input.split(' ').collect();

        let cmd = cmd_line[0];
        let args = &cmd_line[1..];

        if cmd == "exit" {
            if !args.is_empty() {
                match cmd_line[1].parse() {
                    Ok(status) => std::process::exit(status),
                    Err(_) => eprintln!("Status code invalid: {}", cmd_line[1]),
                }
            } else {
                std::process::exit(0);
            }
        } else {
            println!("{input}: command not found");
        }
    }
}
