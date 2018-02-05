extern crate chickenize;
use chickenize::*;

use std::env;
use std::fs;
use std::io;

// To enter text from stdin:
// cargo run --bin chickenize -
//
// To read from a file:
// cargo run --bin chickenize file.txt

/// A chickenize utility
fn main() {
  let input = env::args().nth(1).unwrap();
  let mut rdr: Box<io::Read> = if input == "-" {
    Box::new(io::stdin())
  } else {
    Box::new(fs::File::open(input).unwrap())
  };
  let mut buffer = String::new();
  rdr.read_to_string(&mut buffer).unwrap_or_default();

  println!("{:?}", buffer.chicken())
}
