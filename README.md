# Chickenize for Rust
[![Build Status](https://secure.travis-ci.org/dginev/rust-chickenize.png?branch=master)](http://travis-ci.org/dginev/rust-chickenize)
[![License](http://img.shields.io/badge/license-MIT-blue.svg)](https://raw.githubusercontent.com/dginev/rust-chickenize/master/LICENSE)
[![crates.io](http://meritbadge.herokuapp.com/chickenize)](https://crates.io/crates/chickenize)

Chickenize everything

### Example

```rust
extern crate chickenize;
use chickenize::Chickenize;

let example = "The Vice-president didn't like his style.";
let chickenized = example.chicken();
// chickenized == "chicken chicken-chicken chicken'chicken chicken chicken chicken."
```