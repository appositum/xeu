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

pub fn execute(cmd: &str, args_str: String) -> io::Result<()> {
    let args: Vec<String> = parse_args(args_str);

    match cmd {
        "cd" => builtins::cmd_cd(args),
        "echo" => builtins::cmd_echo(args),
        "exit" => builtins::cmd_exit(args),
        "pwd" => builtins::cmd_pwd(),
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

fn parse_args(input: String) -> Vec<String> {
    let mut in_single_quotes = false;
    let mut in_double_quotes = false;

    let mut current_word: Vec<u8> = vec![];
    let mut all_words: Vec<String> = vec![];

    let mut escaped = false;

    let mut iter = input.bytes().peekable();

    loop {
        if let Some(byte) = iter.next() {
            if escaped {
                current_word.push(byte);
                escaped = false;
                continue;
            }

            match byte {
                b'\'' => {
                    if in_double_quotes {
                        current_word.push(byte);
                    } else {
                        in_single_quotes = !in_single_quotes;
                    }
                },
                b'\"' => {
                    if in_single_quotes {
                        current_word.push(byte);
                    } else {
                        in_double_quotes = !in_double_quotes;
                    }
                },
                b'\\' => {
                    if in_double_quotes {
                        if let Some(&peek) = iter.peek() {
                            if peek == b'\"' {
                                escaped = true;
                            } else {
                                current_word.push(byte);
                            }
                        }
                    } else if in_single_quotes {
                        if let Some(&peek) = iter.peek() {
                            if peek == b'\'' {
                                escaped = true;
                            } else {
                                current_word.push(byte);
                            }
                        }
                    } else if !in_single_quotes && !in_double_quotes {
                        escaped = true;
                    }
                },
                b' ' => {
                    if !in_single_quotes && !in_double_quotes {
                        if !current_word.is_empty() {
                            let word = String::from_utf8(current_word.clone()).unwrap();
                            all_words.push(word);
                            current_word.clear();
                        }
                    } else {
                        current_word.push(byte);
                    }
                },
                _ => {
                    current_word.push(byte);
                },
            }
        } else {
            break;
        }
    }

    if !current_word.is_empty() {
        let word = String::from_utf8(current_word.clone()).unwrap();
        all_words.push(word);
    }

    return all_words;
}
