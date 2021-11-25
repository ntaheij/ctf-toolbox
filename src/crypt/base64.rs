extern crate base64;

use crate::colors::Colors::*;

pub fn execute(mut args: std::str::SplitWhitespace) {
  match args.next() {
    Some("e") | Some("encode") => {
        let output = encode(args.next().unwrap());
        println!("{}", output);
    }
    Some("d") | Some("decode") => {
        let output = decode(args.next().unwrap());
        println!("{}", output);
    }

    Some(_) | None => {
        eprintln!(
            "{}Incorrect Usage.\n{}{}Please use: b64 [encode/decode] [value] {}", 
                DARKRED.value(), RESET.value(), RED.value(), RESET.value()
        );
    }
}
}

pub fn encode(to_encode: &str) -> String {
  return base64::encode(to_encode.as_bytes());
}

pub fn decode(to_decode: &str) -> String {
  return String::from_utf8(base64::decode(to_decode).unwrap()).unwrap();
}