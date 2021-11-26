use md5;

use crate::colors::Colors::*;
use crate::utils::args::get_args_spaces;
use crate::general::help::Commands;

pub fn execute(mut args: std::str::SplitWhitespace) {

  match args.next() {
    Some(_) => {
      let output = encode(get_args_spaces(args));
      println!("{:x}", output);
    }

    None => {
      eprintln!(
        "{}Incorrect Usage.\n{0}{1}Please use: {2} {1}", 
            DARKRED.value(), RESET.value(), Commands::MD5.value()
    );
    }
  }
}

pub fn encode(to_encode: String) -> md5::Digest {
   return md5::compute(to_encode);
}