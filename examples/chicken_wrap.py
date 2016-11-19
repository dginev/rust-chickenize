# To setup this script, you just need a modern Python.
# Create the libchickenize.so dynamic library by running:
#   cargo build --release
# And then execute this Perl example from the top directory as:
#   python examples/chicken_wrap.py

from ctypes import cdll, c_char_p
from sys import platform

if platform == 'darwin':
    prefix = 'lib'
    ext = 'dylib'
elif platform == 'win32':
    prefix = ''
    ext = 'dll'
else:
    prefix = 'lib'
    ext = 'so'

lib = cdll.LoadLibrary('target/release/{}chickenize.{}'.format(prefix, ext))

chickenize = lib.chickenize
chickenize.restype = c_char_p
chickenize.argtypes = [c_char_p]

buffalo = lib.buffalo
buffalo.restype = c_char_p
buffalo.argtypes = [c_char_p]

lorem_ipsum = lib.lorem_ipsum
lorem_ipsum.restype = c_char_p
lorem_ipsum.argtypes = [c_char_p]

anonymize = lib.anonymize
anonymize.restype = c_char_p
anonymize.argtypes = [c_char_p, c_char_p]


fancy = "The Vice-president didn't like his style"

print "\nChickenized:\n", chickenize(fancy)
print "\nBuffalo:\n", buffalo(fancy)
print "\nLorem:\n", lorem_ipsum(fancy)
print "\nAnonymize (meow):\n", anonymize(fancy, "meow")
print
