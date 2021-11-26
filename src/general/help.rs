#[allow(dead_code)]
use enum_iterator::IntoEnumIterator;

use crate::colors::Colors::*;

#[derive(Debug, IntoEnumIterator, PartialEq)]
pub enum Commands {
  B64,
  SHA1,
  MD5
}

impl Commands {
  pub const fn value(&self) -> &str {
    match self {
      Commands::B64 => "b64 [encode/decode] [value]",
      Commands::SHA1 => "sha1 [value]",
      Commands::MD5 => "md5 [value]",
    }
  }
}

pub fn execute(mut args: std::str::SplitWhitespace) {
  match args.next() {
    Some("c") | Some("crypt") => {
        
    }

    Some("all") => {
      print!("{}{}", RESET.value(), YELLOW.value());
      for command in Commands::into_enum_iter() {
        println!("-> {}", command.value().to_owned());
      }
    }

    _ => {
      println!("{}Help Catogories", DARKGREEN.value());
      print!("{}{}", RESET.value(), YELLOW.value());

      println!("-> help crypt");
      println!("-> help all");
    }
  }
}