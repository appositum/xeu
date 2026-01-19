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
    process::{
        self,
        Stdio,
    },
};

use is_executable::is_executable;

mod builtins;

struct Command {
    name: String,
    args: Vec<String>,
    redirection: Option<Box<Command>>,
}

pub fn execute(input: String) -> io::Result<()> {
    match parse_args(input) {
        Err(e) => {
            eprintln!("{e}");
            return Ok(());
        },
        Ok(command) => {
            let cmd = command.name.as_str();
            let args = command.args;

            match cmd {
                "cd" => builtins::cmd_cd(args),
                "echo" => builtins::cmd_echo(args),
                "exit" => builtins::cmd_exit(args),
                "pwd" => builtins::cmd_pwd(),
                "type" => builtins::cmd_type(args),
                _ => {
                    if let Some(_) = get_bin_path(cmd) {
                        let process = process::Command::new(cmd).args(args).spawn()?;

                        let output = process.wait_with_output()?;

                        io::stdout().write_all(&output.stdout)?;
                        io::stderr().write_all(&output.stderr)?;
                    } else {
                        println!("{cmd}: command not found");
                    }
                },
            }
        },
    }

    return Ok(());
}

fn get_bin_path(cmd: &str) -> Option<String> {
    let directories: Vec<PathBuf> = split_paths(&var("PATH").unwrap()).collect();

    for dir in directories {
        if let Ok(entries) = read_dir(dir) {
            for entry in entries {
                if let Ok(e) = entry {
                    if e.file_name() == cmd && is_executable(e.path()) {
                        return Some(String::from(e.path().to_str().unwrap()));
                    }
                }
            }
        }
    }

    return None;
}

fn parse_args(input: String) -> Result<Command, &'static str> {
    let mut in_single_quotes = false;
    let mut in_double_quotes = false;

    let mut current_word: Vec<u8> = vec![];
    let mut all_words: Vec<String> = vec![];
    let mut commands_with_redirections: Vec<Vec<String>> = vec![];

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
                    if let Some(&peek) = iter.peek() {
                        match peek {
                            b'\"' => {
                                if in_double_quotes || !in_single_quotes {
                                    escaped = true;
                                } else {
                                    current_word.push(byte);
                                }
                            },
                            b'\'' => {
                                if in_single_quotes || !in_double_quotes {
                                    escaped = true;
                                } else {
                                    current_word.push(byte);
                                }
                            },
                            b'\\' => {
                                escaped = true;
                            },
                            _ => {
                                if !in_single_quotes && !in_double_quotes {
                                    escaped = true;
                                } else {
                                    current_word.push(byte);
                                }
                            },
                        }
                    }
                },
                b'>' => {
                    if !in_single_quotes && !in_double_quotes {
                        commands_with_redirections.push(all_words.clone());
                        all_words.clear();
                    } else {
                        current_word.push(byte);
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
        commands_with_redirections.push(all_words.clone());
    }

    println!("commands_with_redirections: {commands_with_redirections:?}");
    println!("all_words: {all_words:?}");

    if all_words.is_empty() {
        // TODO: this should send a status code 123.
        // And ideally, it would also return `Err`, not `Ok`.
        // It might be time to create an error module.
        return Err("xeu: The expanded command was empty");
    }

    return Ok(Command {
        name: all_words[0].clone(),
        args: all_words[1..].to_vec(),
        // TODO: implement the redirections
        redirection: None,
    });
}
