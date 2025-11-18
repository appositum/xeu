use std::{
    env::{
        split_paths,
        var,
    },
    fs::read_dir,
    io::{
        self,
        Write,
    },
    path::PathBuf,
    process::Command,
};

use is_executable::is_executable;

mod builtins;

pub fn execute(cmd: &str, args: &[&str]) -> io::Result<()> {
    match cmd {
        "echo" => builtins::cmd_echo(args),
        "exit" => builtins::cmd_exit(args),
        "type" => builtins::cmd_type(args),
        _ => {
            if let Some(_) = get_bin_path(cmd) {
                let output = Command::new(cmd).args(args).output()?;

                io::stdout().write_all(&output.stdout)?;
                io::stderr().write_all(&output.stderr)?;
            } else {
                println!("{cmd}: command not found");
            }
        },
    }

    return Ok(());
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
