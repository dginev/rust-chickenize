extern crate chickenize;
use chickenize::Chickenize;

#[test]
fn chickenize_string() {
  let hello = "hello world!";
  assert_eq!(hello.chicken(), "chicken chicken!");

  let hello_string = String::from(hello);
  assert_eq!(hello_string.chicken(), "chicken chicken!");

  let fancy = "The Vice-president didn't like his style";
  assert_eq!(
    fancy.chicken(),
    "chicken chicken-chicken chicken'chicken chicken chicken chicken"
  );
}

#[test]
fn chickenize_int_vec() {
  let v = vec![1, 2, 3];
  let expected_v = "chicken chicken chicken";
  let chicken_v = v.chicken();
  assert_eq!(chicken_v, expected_v);
}

#[test]
fn chickenize_math() {
  let math = "1 + 2 = 3";
  assert_eq!(math.chicken(), "chicken + chicken = chicken");
}

#[test]
fn buffalo_string() {
  let hello = "hello again, my friend.";
  assert_eq!(hello.buffalo(), "buffalo buffalo, buffalo buffalo.");
}

#[test]
fn anonymize_string() {
  let hello = "hello again, my friend.";
  assert_eq!(hello.anonymize("work"), "work work, work work.");
}

#[test]
fn lorem_ipsum_string() {
  let hello = "hello again, my friend.";
  assert_eq!(hello.lorem_ipsum(), "Lorem ipsum, dolor sit.");
}

#[test]
fn readme_example_test() {
  let example = "The Vice-president didn't like his style.";
  let chickenized = example.chicken();
  assert_eq!(
    chickenized,
    "chicken chicken-chicken chicken'chicken chicken chicken chicken."
  );

  let buffalaxed = example.buffalo();
  assert_eq!(
    buffalaxed,
    "buffalo buffalo-buffalo buffalo'buffalo buffalo buffalo buffalo."
  );

  let anonymized = example.anonymize("secret");
  assert_eq!(
    anonymized,
    "secret secret-secret secret'secret secret secret secret."
  );

  let lorem_ipsum = example.lorem_ipsum();
  assert_eq!(
    lorem_ipsum,
    "Lorem ipsum-dolor sit'amet consectetur adipiscing elit."
  );
}
