# To setup this script, you need your everyday Ruby, with ffi installed, for example via:
#   gem install ffi
# Create the libchickenize.so dynamic library by running:
#   cargo build --release
# And then execute this Perl example from the top directory as:
#   ruby examples/chicken_wrap.rb

require 'ffi'

if RUBY_PLATFORM.include?('darwin')
  EXT = 'dylib'
else
  EXT = 'so'
end

module Chickenize
  extend FFI::Library
  ffi_lib 'target/release/libchickenize.' + EXT

  attach_function :chickenize, [:string], :string
  attach_function :buffalo, [:string], :string
  attach_function :lorem_ipsum, [:string], :string
  attach_function :anonymize, [:string,:string], :string
end

fancy = "The Vice-president didn't like his style"
# currently Rust will deallocate input arguments, need to be careful
#my fancy = master_fancy

puts ""
puts "Chickenized:"
puts Chickenize.chickenize(fancy)
#fancy = master_fancy

puts ""
puts "Buffalo:"
puts Chickenize.buffalo(fancy)
#fancy = master_fancy

puts ""
puts "Lorem:"
puts Chickenize.lorem_ipsum(fancy)
#fancy = master_fancy

puts ""
puts "Anonymized (meow):"
puts Chickenize.anonymize(fancy, "meow")
puts ""