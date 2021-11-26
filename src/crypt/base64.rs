extern crate base64;

use crate::colors::Colors::*;
use crate::utils::args::get_args_spaces;
use crate::general::help::Commands;

pub fn execute(mut args: std::str::SplitWhitespace) {
  match args.next() {
    Some("e") | Some("encode") => {
        let output = encode(get_args_spaces(args));
        println!("{}", output);
    }
    Some("d") | Some("decode") => {
        let output = decode(get_args_spaces(args));
        println!("{}", output);
    }

    _ => {
        eprintln!(
            "{}Incorrect Usage.\n{0}{1}Please use: {2} {1}", 
                DARKRED.value(), RESET.value(), Commands::B64.value()
        );
    }
  }
}

pub fn encode(to_encode: String) -> String {
  return base64::encode(to_encode.as_bytes());
}

pub fn decode(to_decode: String) -> String {
  return String::from_utf8(base64::decode(to_decode).unwrap()).unwrap();
}