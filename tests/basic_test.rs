extern crate chickenize;
use chickenize::Chickenize;

#[test]
fn chickenize_string() {
  let hello = "hello world!";
  assert_eq!(hello.chicken("chicken"), "chicken chicken!");

  let hello_string = String::from(hello);
  assert_eq!(hello_string.chicken("chicken"), "chicken chicken!");

  let fancy = "The Vice-president didn't like his style";
  assert_eq!(fancy.chicken("chicken"),
             "chicken chicken-chicken chicken'chicken chicken chicken chicken");
}

#[test]
fn chickenize_int_vec() {
  let v = vec![1, 2, 3];
  let expected_v = "chicken chicken chicken";
  let chicken_v = v.chicken("chicken");
  assert_eq!(chicken_v, expected_v);
}

#[test]
fn chickenize_math() {
  let math = "1 + 2 = 3";
  assert_eq!(math.chicken("chicken"), "chicken + chicken = chicken");
}

#[test]
fn buffalo_chickenize_string() {
  let buffalo_chicken_exclamation_point_expected = ["chicken chicken!", "chicken buffalo!", "buffalo buffalo!", "buffalo chicken!"];

  let hello = "hello world!";
  assert!(buffalo_chicken_exclamation_point_expected.contains(&&hello.buffalo_chicken(&["chicken", "buffalo"])[..]));

  let hello_string = String::from(hello);
  assert!(buffalo_chicken_exclamation_point_expected.contains(&&hello_string.buffalo_chicken(&["chicken", "buffalo"])[..]));

  let fancy = "The Vice-president didn't like his style";
  let mut vice_president_expected = Vec::new();
  for x in 0..256 {
    let c = format!("{:08b}", x).chars().map(|c| if c == '0' {"chicken"} else {"buffalo"}).collect::<Vec<_>>();
    vice_president_expected.push(format!("{} {}-{} {}'{} {} {} {}", c[0], c[1], c[2], c[3], c[4], c[5], c[6], c[7]));
  }
  assert!(vice_president_expected.contains(&fancy.buffalo_chicken(&["chicken", "buffalo"])));
}

#[test]
fn buffalo_chickenize_int_vec() {
  let v = vec![1, 2, 3];
  let expected_v = [
    "chicken chicken chicken",
    "chicken chicken buffalo",
    "chicken buffalo chicken",
    "chicken buffalo buffalo",
    "buffalo chicken chicken",
    "buffalo chicken buffalo",
    "buffalo buffalo chicken",
    "buffalo buffalo buffalo",
  ];
  let chicken_v = v.buffalo_chicken(&["chicken", "buffalo"]);
  assert!(expected_v.contains(&&chicken_v[..]));
}

#[test]
fn buffalo_chickenize_math() {
  let math = "1 + 2 = 3";
  let math_buffalo_chicken = [
    "chicken + chicken = chicken",
    "chicken + chicken = buffalo",
    "chicken + buffalo = chicken",
    "chicken + buffalo = buffalo",
    "buffalo + chicken = chicken",
    "buffalo + chicken = buffalo",
    "buffalo + buffalo = chicken",
    "buffalo + buffalo = buffalo",
  ];
  assert!(math_buffalo_chicken.contains(&&math.buffalo_chicken(&["chicken", "buffalo"])[..]));
}
