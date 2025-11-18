use std::{
    env::{
        current_dir,
        set_current_dir,
        var,
    },
    path::PathBuf,
};

const COMMANDS: &[&str] = &["cd", "echo", "exit", "pwd", "type"];

pub fn cmd_cd(args: &[&str]) {
    if args.len() > 1 {
        eprintln!("cd: too many arguments");
        return;
    }

    let arg = args[0];
    let mut destination = PathBuf::new();
    let home = var("HOME").unwrap();

    if arg == "~" || arg == "~/" {
        destination.push(home);
    } else if arg.starts_with("~/") {
        let rest = &arg[2..];
        destination.push(home);
        destination.push(rest);
    } else {
        destination = current_dir().unwrap();

        destination.push(arg);
    }

    if let Err(_) = set_current_dir(destination) {
        println!("cd: {}: No such file or directory", arg);
    }
}

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

pub fn cmd_pwd() {
    let cdir = current_dir().unwrap();
    println!("{}", cdir.display());
}

pub fn cmd_type(args: &[&str]) {
    if !args.is_empty() {
        let arg = &args[0];

        if COMMANDS.contains(&arg) {
            println!("{arg} is shell builtin");
        } else {
            if let Some(cmd_path) = crate::get_bin_path(arg) {
                println!("{arg} is {cmd_path}");
            } else {
                println!("{arg}: not found");
            }
        }
    }
}
