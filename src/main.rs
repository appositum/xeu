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
        println!("{input}: command not found")
    }
}
