use sha1::{Sha1};

use crate::colors::Colors::*;
use crate::utils::args::get_args_spaces;

pub fn execute(mut args: std::str::SplitWhitespace) {
  match args.next() {
    Some(_) => {
      let output = encode(get_args_spaces(args));
      println!("{}", output);
    }

    None => {
        eprintln!(
            "{}Incorrect Usage.\n{}{}Please use: sha1 [value] {}", 
                DARKRED.value(), RESET.value(), RED.value(), RESET.value()
        );
    }
  }
}

pub fn encode(to_encode: String) -> String {
  let mut hasher = Sha1::new();
  hasher.update(to_encode.as_bytes());

  return hasher.digest().to_string();
}