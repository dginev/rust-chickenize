extern crate rand;
use rand::{Rng, thread_rng};

pub trait Chickenize {
  fn chicken(&self, buffalo: &str) -> String {
    String::from(buffalo)
  }

  fn buffalo_chicken(&self, buffalos: &[&str]) -> String {
    String::from(*thread_rng().choose(buffalos).unwrap())
  }
}

impl Chickenize for bool {}
impl Chickenize for char {}
impl Chickenize for i8 {}
impl Chickenize for i16 {}
impl Chickenize for i32 {}
impl Chickenize for i64 {}
impl Chickenize for u8 {}
impl Chickenize for u16 {}
impl Chickenize for u32 {}
impl Chickenize for u64 {}
impl Chickenize for isize {}
impl Chickenize for usize {}
impl Chickenize for f32 {}
impl Chickenize for f64 {}

impl<'a> Chickenize for &'a str {
  fn chicken(&self, buffalo: &str) -> String {
    self.buffalo_chicken(&[buffalo])
  }

  fn buffalo_chicken(&self, buffalos: &[&str]) -> String {
    let mut rng = thread_rng();
    let text_iterator = self.chars().peekable();
    let mut chickenized = String::new();
    let mut seen_letter = false;
    for c in text_iterator {
      if !c.is_alphanumeric() {
        if seen_letter {
          chickenized.push_str(rng.choose(buffalos).unwrap());
          seen_letter = false;
        }
        chickenized.push(c);
      } else {
        seen_letter = true;
      }
    }
    // trailing words
    if seen_letter {
      chickenized.push_str(rng.choose(buffalos).unwrap());
    }
    return chickenized;
  }
}

impl Chickenize for String {
  fn chicken(&self, buffalo: &str) -> String {
    self.as_str().chicken(buffalo)
  }

  fn buffalo_chicken(&self, buffalos: &[&str]) -> String {
    self.as_str().buffalo_chicken(buffalos)
  }
}

impl Chickenize for Vec<String> {
  fn chicken(&self, buffalo: &str) -> String {
    let chickenized_vec: Vec<String> = self.iter().map(|x| x.chicken(buffalo)).collect();
    chickenized_vec.join(" ")
  }

  fn buffalo_chicken(&self, buffalos: &[&str]) -> String {
    let chickenized_vec: Vec<String> = self.iter().map(|x| x.buffalo_chicken(buffalos)).collect();
    chickenized_vec.join(" ")
  }
}

impl Chickenize for Vec<i32> {
  fn chicken(&self, buffalo: &str) -> String {
    let chickenized_vec: Vec<String> = self.iter().map(|x| x.chicken(buffalo)).collect();
    chickenized_vec.join(" ")
  }

  fn buffalo_chicken(&self, buffalos: &[&str]) -> String {
    let chickenized_vec: Vec<String> = self.iter().map(|x| x.buffalo_chicken(buffalos)).collect();
    chickenized_vec.join(" ")
  }
}
