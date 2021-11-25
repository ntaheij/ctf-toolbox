use md5;

use crate::colors::Colors::*;
use crate::utils::args::get_args_spaces;

pub fn execute(mut args: std::str::SplitWhitespace) {

  match args.next() {
    Some(_) => {
      let output = encode(get_args_spaces(args));
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

pub fn encode(to_encode: String) -> md5::Digest {
   return md5::compute(to_encode);
}