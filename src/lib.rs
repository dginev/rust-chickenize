static CHICKEN: &'static str = "chicken";
pub trait Chickenize {
  fn chicken(&self) -> String {
    String::from(CHICKEN)
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
  fn chicken(&self) -> String {
    let text_iterator = self.chars().peekable();
    let mut chickenized = String::new();
    let mut seen_letter = false;
    for c in text_iterator {
      if !c.is_alphanumeric() {
        if seen_letter {
          chickenized.push_str(CHICKEN);
          seen_letter = false;
        }
        chickenized.push(c);
      } else {
        seen_letter = true;
      }
    }
    // trailing words
    if seen_letter {
      chickenized.push_str(CHICKEN);
    }
    return chickenized;
  }
}

impl Chickenize for String {
  fn chicken(&self) -> String {
    self.as_str().chicken()
  }
}

impl Chickenize for Vec<String> {
  fn chicken(&self) -> String {
    let chickenized_vec: Vec<String> = self.iter().map(|x| x.chicken()).collect();
    chickenized_vec.join(" ")
  }
}

impl Chickenize for Vec<i32> {
  fn chicken(&self) -> String {
    let chickenized_vec: Vec<String> = self.iter().map(|x| x.chicken()).collect();
    chickenized_vec.join(" ")
  }
}
