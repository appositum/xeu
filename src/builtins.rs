const COMMANDS: &[&str] = &["echo", "exit", "type"];

pub fn cmd_echo(args: &[&str]) {
    if args.is_empty() {
        println!("");
    } else {
        println!("{}", args.join(" "));
    }
}

pub fn cmd_exit(args: &[&str]) {
    if args.is_empty() {
        std::process::exit(0)
    }

    match args[0].parse() {
        Ok(status) => std::process::exit(status),
        Err(_) => eprintln!("Status code invalid: {}", args[0]),
    }
}

pub fn cmd_type(args: &[&str]) {
    if !args.is_empty() {
        let arg = &args[0];

        if COMMANDS.contains(&arg) {
            println!("{arg} is shell builtin");
        } else {
            println!("{arg}: not found");
        }
    }
}
