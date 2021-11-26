#[allow(dead_code)]
use enum_iterator::IntoEnumIterator;

#[derive(Debug, IntoEnumIterator, PartialEq)]
pub enum Colors {
  RESET,
  RED,
  DARKRED,
  CYAN,
  DARKCYAN,
  GREEN,
  DARKGREEN,
  YELLOW,
  DARKYELLOW,
  BLUE,
  DARKBLUE,
  MAGENTA,
  DARKMAGENTA,
  WHITE,
  DARKWHITE,
  BOLD,
  UNDERLINE,
  REVERSE,
  HIDDEN,
}

impl Colors {
  pub const fn value(&self) -> &str {
    match self {
      Colors::RESET => "\x1b[0m",
      Colors::RED => "\x1b[31m",
      Colors::DARKRED => "\x1b[31;1m",
      Colors::CYAN => "\x1b[36m",
      Colors::DARKCYAN => "\x1b[36;1m",
      Colors::GREEN => "\x1b[32m",
      Colors::DARKGREEN => "\x1b[32;1m",
      Colors::YELLOW => "\x1b[33m",
      Colors::DARKYELLOW => "\x1b[33;1m",
      Colors::BLUE => "\x1b[34m",
      Colors::DARKBLUE => "\x1b[34;1m",
      Colors::MAGENTA => "\x1b[35m",
      Colors::DARKMAGENTA => "\x1b[35;1m",
      Colors::WHITE => "\x1b[37m",
      Colors::DARKWHITE => "\x1b[37;1m",
      Colors::BOLD => "\x1b[1m",
      Colors::UNDERLINE => "\x1b[4m",
      Colors::REVERSE => "\x1b[7m",
      Colors::HIDDEN => "\x1b[8m",
    }
  }
}

pub fn show_colors() {
  for color in Colors::into_enum_iter() {
    println!("{}", color.value().to_owned() + "This is " + color.value() + "colored text" + Colors::RESET.value());
  }
}