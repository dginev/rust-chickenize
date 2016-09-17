static CHICKEN: &'static str = "chicken";
static BUFFALO: &'static str = "buffalo";

pub trait Chickenize {
  fn chicken(&self) -> String {
    String::from(CHICKEN)
  }

  fn buffalo(&self) -> String {
    String::from(BUFFALO)
  }

  fn anonymize(&self, replacement: &str) -> String {
    String::from(replacement)
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

// We are using an Anonymize macro here to avoid argument passing in runtime and keep
//    the concrete (standard) chickenizations as performant as possible.
#[macro_export]
macro_rules! Anonymize(
  ($text:expr, $replacement:expr) => (
  {
    let text_iterator = $text.chars().peekable();
    let mut chickenized = String::new();
    let mut seen_letter = false;
    for c in text_iterator {
      if !c.is_alphanumeric() {
        if seen_letter {
          chickenized.push_str($replacement);
          seen_letter = false;
        }
        chickenized.push(c);
      } else {
        seen_letter = true;
      }
    }
    if seen_letter {
      chickenized.push_str($replacement);
    }
    return chickenized;
  }
  )
);

impl<'a> Chickenize for &'a str {
  fn chicken(&self) -> String {
    Anonymize!(self, CHICKEN)
  }
  fn buffalo(&self) -> String {
    Anonymize!(self, BUFFALO)
  }

  fn anonymize(&self, replacement: &str) -> String {
    Anonymize!(self, replacement)
  }
}

impl Chickenize for String {
  fn chicken(&self) -> String {
    self.as_str().chicken()
  }
  fn buffalo(&self) -> String {
    self.as_str().buffalo()
  }
  fn anonymize(&self, replacement: &str) -> String {
    self.as_str().anonymize(replacement)
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
