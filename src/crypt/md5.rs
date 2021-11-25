use md5;

use crate::colors::Colors::*;

pub fn execute(mut args: std::str::SplitWhitespace) {
  match args.next() {
    Some(input) => {
      let output = encode(input);
      println!("{:x}", output);
    }

    None => {
        eprintln!(
            "{}Incorrect Usage.\n{}{}Please use: md5 [value] {}", 
                DARKRED.value(), RESET.value(), RED.value(), RESET.value()
        );
    }
  }
}

pub fn encode(to_encode: &str) -> md5::Digest {
   return md5::compute(to_encode);
}