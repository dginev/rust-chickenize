use chickenize::*;
use wasm_bindgen_test::*;

wasm_bindgen_test_configure!(run_in_browser);

#[wasm_bindgen_test]
fn chickenize_string() {
  let hello = "hello world!";
  assert_eq!(wasm_str_chicken(hello), "chicken chicken!");

  let hello_string = String::from(hello);
  assert_eq!(wasm_string_chicken(hello_string), "chicken chicken!");

  let fancy = "The Vice-president didn't like his style";
  assert_eq!(wasm_str_chicken(fancy), "chicken chicken-chicken chicken'chicken chicken chicken chicken");
}

#[wasm_bindgen_test]
fn chickenize_int_vec() {
  let v = vec![1, 2, 3];
  let expected_v = "chicken chicken chicken";
  let chicken_v = wasm_vec_chicken(v);
  assert_eq!(chicken_v, expected_v);
}

#[wasm_bindgen_test]
fn chickenize_math() {
  let math = "1 + 2 = 3";
  assert_eq!(wasm_str_chicken(math), "chicken + chicken = chicken");
}

#[wasm_bindgen_test]
fn buffalo_string() {
  let hello = "hello again, my friend.";
  assert_eq!(wasm_str_buffalo(hello), "buffalo buffalo, buffalo buffalo.");
}

// #[wasm_bindgen_test]
// fn anonymize_string() {
//   let hello = "hello again, my friend.";
//   assert_eq!(wasm_str_anon(hello, "work"), "work work, work work.");
// }

#[wasm_bindgen_test]
fn lorem_ipsum_string() {
  let hello = "hello again, my friend.";
  assert_eq!(wasm_str_lorem(hello), "Lorem ipsum, dolor sit.");
}

#[wasm_bindgen_test]
fn readme_example_test() {
  let example = "The Vice-president didn't like his style.";
  let chickenized = wasm_str_chicken(example);
  assert_eq!(chickenized, "chicken chicken-chicken chicken'chicken chicken chicken chicken.");

  let buffalaxed = wasm_str_buffalo(example);
  assert_eq!(buffalaxed, "buffalo buffalo-buffalo buffalo'buffalo buffalo buffalo buffalo.");

  // let anonymized = wasm_str_anon(example, "secret");
  // assert_eq!(anonymized, "secret secret-secret secret'secret secret secret secret.");

  let lorem_ipsum = wasm_str_lorem(example);
  assert_eq!(lorem_ipsum, "Lorem ipsum-dolor sit'amet consectetur adipiscing elit.");
}
