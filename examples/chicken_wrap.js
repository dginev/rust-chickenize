// To setup this script, you need node with ffi installed, for example via:
//   npm install ffi
// Create the libchickenize.so dynamic library by running:
//   cargo build --release
// And then execute this Perl example from the top directory as:
//   node examples/chicken_wrap.js

var ffi = require('ffi');

var lib = ffi.Library('target/release/libchickenize', {
  'chickenize': [ 'string', [ 'string' ] ],
  'buffalo': [ 'string', [ 'string' ] ],
  'lorem_ipsum': [ 'string', [ 'string' ] ],
  'anonymize': [ 'string', [ 'string', 'string' ] ],
});

var fancy = "The Vice-president didn't like his style";

console.log("\nChickenized:\n", lib.chickenize(fancy));
console.log("\nBuffalo:\n", lib.buffalo(fancy));
console.log("\nLorem:\n", lib.lorem_ipsum(fancy));
console.log("\nAnonymize (meow):\n", lib.anonymize(fancy, "meow"));
