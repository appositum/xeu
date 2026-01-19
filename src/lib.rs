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

pub fn execute(input: String) -> io::Result<()> {
    let parsed_cmd_line: Vec<String> = parse_args(input);

    if parsed_cmd_line.is_empty() {
        // TODO: this should send a status code 123.
        // And ideally, it would also return `Err`, not `Ok`.
        // It might be time to create an error module.
        println!("xeu: The expanded command was empty");
        return Ok(());
    }

    let cmd = parsed_cmd_line[0].as_str();

    let args: Vec<String> = parsed_cmd_line[1..].to_vec();

    match cmd {
        "cd" => builtins::cmd_cd(args),
        "echo" => builtins::cmd_echo(args),
        "exit" => builtins::cmd_exit(args),
        "pwd" => builtins::cmd_pwd(),
        "type" => builtins::cmd_type(args),
        _ => {
            if let Some(_) = get_bin_path(cmd) {
                let process = Command::new(cmd).args(args).spawn()?;
                let output = process.wait_with_output()?;

                io::stdout().write_all(&output.stdout)?;
                io::stderr().write_all(&output.stderr)?;
            } else {
                println!("{cmd}: command not found");
            }
        },
    }

    return Ok(());
}

fn get_bin_path(cmd: &str) -> Option<String> {
    let directories: Vec<PathBuf> =
        split_paths(&var("PATH").unwrap()).collect();

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

fn parse_args(input: String) -> Vec<String> {
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
                            let word = String::from_utf8(current_word.clone())
                                .unwrap();
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

    // TODO: use custom `Command` struct
    println!("commands_with_redirections: {commands_with_redirections:?}");
    println!("all_words: {all_words:?}");

    return all_words;
}
