// If you want to try something cool, follow the instructions to install asm.js here:
// https://users.rust-lang.org/t/compiling-to-the-web-with-rust-and-emscripten/7627
//
// and natively compile to JS via cargo:
//
// cargo build --release --target=asmjs-unknown-emscripten --example chicken_wrap
//
// Try it out via:
// node target/asmjs-unknown-emscripten/release/examples/chicken_wrap.js
//
extern crate chickenize;
use chickenize::*;

fn main() {
  let example = "The Vice-president didn't like his style";

  println!("");
  println!("Chickenized:");
  println!("{:?}",example.chicken());

  println!("");
  println!("Buffalo:");
  println!("{:?}",example.buffalo());

  println!("");
  println!("Lorem:");
  println!("{:?}",example.lorem_ipsum());

  println!("");
  println!("Anonymized (meow):");
  println!("{:?}",example.anonymize("meow"));
  println!("");
}