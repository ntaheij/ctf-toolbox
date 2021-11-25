// TODO: Implement more efficient way to parse args
pub fn get_args_spaces(args: std::str::SplitWhitespace) -> String {
  let mut result: String = String::new();
  for arg in args {
    result.push_str(arg);
    result.push_str(" ");
  }

  result.pop();

  return result;
}