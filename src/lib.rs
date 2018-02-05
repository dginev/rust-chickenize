use std::ffi::{CStr, CString};
use std::os::raw::c_char;

mod loremipsum;

static CHICKEN: &'static str = "chicken";
static BUFFALO: &'static str = "buffalo";

pub trait Chickenize {
  fn chicken(&self) -> String { String::from(CHICKEN) }

  fn buffalo(&self) -> String { String::from(BUFFALO) }

  fn anonymize(&self, replacement: &str) -> String { String::from(replacement) }

  fn lorem_ipsum(&self) -> String { String::from("Lorem Ipsum") }
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
    chickenized
  }
  )
);

impl<'a> Chickenize for &'a str {
  fn chicken(&self) -> String { Anonymize!(self, CHICKEN) }
  fn buffalo(&self) -> String { Anonymize!(self, BUFFALO) }
  fn anonymize(&self, replacement: &str) -> String { Anonymize!(self, replacement) }
  fn lorem_ipsum(&self) -> String {
    let mut li = loremipsum::Generator::default();
    Anonymize!(self, li.next_word())
  }
}

impl Chickenize for String {
  fn chicken(&self) -> String { self.as_str().chicken() }
  fn buffalo(&self) -> String { self.as_str().buffalo() }
  fn anonymize(&self, replacement: &str) -> String { self.as_str().anonymize(replacement) }
  fn lorem_ipsum(&self) -> String { self.as_str().lorem_ipsum() }
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

#[no_mangle]
pub unsafe extern "C" fn chickenize(value: *const c_char) -> *mut c_char {
  let c_value = CStr::from_ptr(value);
  let chickenized = match c_value.to_str() {
    Ok(value) => value.chicken(),
    _ => String::new(),
  };
  CString::new(chickenized).unwrap().into_raw()
}

#[no_mangle]
pub unsafe extern "C" fn buffalo(value: *const c_char) -> *mut c_char {
  let c_value = CStr::from_ptr(value);
  let buffalo = match c_value.to_str() {
    Ok(value) => value.buffalo(),
    _ => String::new(),
  };
  CString::new(buffalo).unwrap().into_raw()
}

#[no_mangle]
pub unsafe extern "C" fn lorem_ipsum(value: *const c_char) -> *mut c_char {
  let c_value = CStr::from_ptr(value);
  let lorem_ipsum = match c_value.to_str() {
    Ok(value) => value.lorem_ipsum(),
    _ => String::new(),
  };
  CString::new(lorem_ipsum).unwrap().into_raw()
}

#[no_mangle]
pub unsafe extern "C" fn anonymize(
  value: *const c_char,
  replacement: *const c_char,
) -> *mut c_char
{
  let c_value = CStr::from_ptr(value);
  let c_replacement = CStr::from_ptr(replacement);
  let anonymized = match c_value.to_str() {
    Ok(value) => match c_replacement.to_str() {
      Ok(replacement) => value.anonymize(replacement),
      _ => String::new(),
    },
    _ => String::new(),
  };
  CString::new(anonymized).unwrap().into_raw()
}
