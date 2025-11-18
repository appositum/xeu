use std::io::{
    self,
    Write,
};

mod builtins;

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

        match cmd {
            "echo" => builtins::cmd_echo(args),
            "exit" => builtins::cmd_exit(args),
            _ => println!("{cmd}: command not found"),
        }
    }
}
