use sha1::{Sha1};

use crate::colors::Colors::*;
use crate::utils::args::get_args_spaces;
use crate::general::help::Commands;

pub fn execute(mut args: std::str::SplitWhitespace) {
  match args.next() {
    Some(_) => {
      let output = encode(get_args_spaces(args));
      println!("{}", output);
    }

    None => {
      eprintln!(
        "{}Incorrect Usage.\n{0}{1}Please use: {2} {1}", 
            DARKRED.value(), RESET.value(), Commands::SHA1.value()
      );
    }
  }
}

pub fn encode(to_encode: String) -> String {
  let mut hasher = Sha1::new();
  hasher.update(to_encode.as_bytes());

  return hasher.digest().to_string();
}