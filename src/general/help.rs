#[allow(dead_code)]
use enum_iterator::IntoEnumIterator;

#[derive(Debug, IntoEnumIterator, PartialEq)]
pub enum Commands {
  B64,
}

impl Commands {
  pub const fn value(&self) -> &str {
    match self {
      Commands::B64 => "b64 [encode/decode] [value]",
    }
  }
}

pub fn execute() {
  for command in Commands::into_enum_iter() {
    println!("{}", command.value().to_owned());
  }
}