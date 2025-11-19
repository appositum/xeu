use std::io::{
    self,
    Write,
};

use xeu;

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

        match input.split_once(' ') {
            None => {
                let _ = xeu::execute(input.as_str(), String::new());
            },
            Some((cmd, args)) => {
                let _ = xeu::execute(cmd, args.to_string());
            },
        }
    }
}
