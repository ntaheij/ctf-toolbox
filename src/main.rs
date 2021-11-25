use std::env;
use std::io::*;
use std::path::Path;
use std::process::Command;

mod crypt;
mod colors;

use colors::Colors::*;

const PREFIX_COLOR: &str = CYAN.value();
const PREFIX: &str = "toolkit>";

fn main() {
    spawn_shell();
}

fn spawn_shell() {
    loop {
        print!("{}{} {}", PREFIX_COLOR, PREFIX, RESET.value());
        stdout().flush().unwrap();

        let mut input = String::new();
        stdin().read_line(&mut input).unwrap();

        // everything after the first whitespace character
        // is interpreted as args to the command
        let mut parts = input.trim().split_whitespace();
        let command = parts.next().unwrap();
        let args = parts;

        match command {
            "cd" => {
                // default to `/` as new directory if one was not provided
                let new_dir = args.peekable().peek().map_or("/", |x| *x);
                let root = Path::new(new_dir);
                if let Err(e) = env::set_current_dir(&root) {
                    println!("{}", e);
                }
            }

            "showcolors" | "colors" => {
                colors::show_colors();
            }

            "base64" | "b64" => {
                crypt::base64::execute(args);
            }

            "sha1" | "s1" => {
                crypt::sha1::execute(args);
            }

            "exit" => return,

            _ => {
                let child = Command::new(command).args(args).spawn();
                if let Err(e) = child {
                    eprintln!("Unkown command -> {}", e);
                } else {
                    child.unwrap().wait().expect("child process error!");
                }
            }
        }
    }
}