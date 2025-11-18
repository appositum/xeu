mod builtins;

pub fn execute(cmd: &str, args: &[&str]) {
    match cmd {
        "echo" => builtins::cmd_echo(args),
        "exit" => builtins::cmd_exit(args),
        "type" => builtins::cmd_type(args),
        _ => println!("{cmd}: command not found"),
    }
}
