#!/usr/bin/env perl

# To setup this script, you need a modern Perl installation with the FFI::Platypus module installed.
# Create the libchickenize.so dynamic library by running:
#   cargo build --release
# And then execute this Perl example from the top directory as:
#   perl examples/chicken_wrap.pl

use v5.10;
use strict;
use warnings;
use FFI::Platypus;

my $ffi = FFI::Platypus->new;
$ffi->lib("target/release/libchickenize.so");
# attach as a xsub and call (much faster)
$ffi->attach( chickenize => ['string'] => 'string' );
$ffi->attach( buffalo => ['string'] => 'string' );
$ffi->attach( lorem_ipsum => ['string'] => 'string' );
$ffi->attach( anonymize => ['string','string'] => 'string' );

my $example = "The Vice-president didn't like his style";

say "";
say "Chickenized:";
say chickenize($example);

say "";
say "Buffalo:";
say buffalo($example);

say "";
say "Lorem:";
say lorem_ipsum($example);

say "";
say "Anonymized (meow):";
say anonymize($example, "meow");
say "";