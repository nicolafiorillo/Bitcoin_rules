use std::io::{self, Write};

use core::std_lib::std_result::StdResult;

#[derive(Debug, PartialEq)]
enum Command {
    Exit,
    Unknown,
}

pub fn print_exit_help() {
    println!("'exit' to close the application");
}

pub fn run_command_line() {
    let mut exit = false;

    while !exit {
        print!("> ");
        let _ = io::stdout().flush();

        let user_command = read_command();
        match translate_command(&user_command) {
            Ok(Command::Exit) => exit = true,
            Ok(Command::Unknown) => println!("Unknown command: {}", user_command),
            Err(e) => println!("Error: {}", e),
        }
    }
}

// Get command from console
fn read_command() -> String {
    let mut line = String::new();
    std::io::stdin()
        .read_line(&mut line)
        .expect("failed to read from command line");
    line
}

// From user string to command
fn translate_command(command: &str) -> StdResult<Command> {
    let normalized_command = command.trim().to_lowercase();

    match normalized_command.as_str() {
        "exit" => Ok(Command::Exit),
        _ => Ok(Command::Unknown),
    }
}

#[cfg(test)]
mod command_line_tests {
    use super::{translate_command, Command};

    #[test]
    fn unknown_command() {
        let command = translate_command("unknown");
        assert_eq!(command.unwrap(), Command::Unknown);
    }

    #[test]
    fn correct_exit() {
        let command = translate_command("exit");
        assert_eq!(command.unwrap(), Command::Exit);
    }

    #[test]
    fn correct_non_trimmed_exit() {
        let command = translate_command(" Exit  ");
        assert_eq!(command.unwrap(), Command::Exit);
    }
}
