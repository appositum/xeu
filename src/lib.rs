use std::{
    env::{
        split_paths,
        var,
    },
    fs::read_dir,
    path::PathBuf,
};

use is_executable::is_executable;

mod builtins;

pub fn execute(cmd: &str, args: &[&str]) {
    match cmd {
        "echo" => builtins::cmd_echo(args),
        "exit" => builtins::cmd_exit(args),
        "type" => builtins::cmd_type(args),
        _ => println!("{cmd}: command not found"),
    }
}

pub fn get_bin_path(cmd: &str) -> Option<String> {
    let directories: Vec<PathBuf> = split_paths(&var("PATH").unwrap()).collect();

    for dir in directories {
        if let Ok(entries) = read_dir(dir) {
            for entry in entries {
                match entry {
                    Ok(e) => {
                        if e.file_name() == cmd && is_executable(e.path()) {
                            return Some(String::from(e.path().to_str().unwrap()));
                        }
                    },
                    Err(_) => {},
                }
            }
        }
    }

    return None;
}
