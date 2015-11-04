# rust-testu01
[![Build Status](https://travis-ci.org/dzamlo/rust-testu01.svg?branch=master)](https://travis-ci.org/dzamlo/rust-testu01)

A rust wrapper to a small subset of [TestU01].
[TestU01]: http://simul.iro.umontreal.ca/testu01/tu01.html

To quote his website :
> TestU01 is a software library, implemented in the ANSI C language, and offering 
> a collection of utilities for the empirical statistical testing of uniform random
> number generators.

Currently, this wrapper only covers:
 * building a "object" which conform to the TestU01 "interface" for generators.
 * running one the predefined tests batteries against a generator implemented in Rust.

In addition to wrapping TestU01 this library provides:
 * Two decorators to help you test your generator more thoroughly. 

## Safety

This binding should be memory safe, with two exceptions:
 * panicking in one of the callback called by TestU01 is potentially unsafe,
 * potential memory unsafety inside TestU01 itself.

## Global Lock

TestU01 is not thread safe. To mitigate any issue, this wrapper has a global lock 
which is acquired before calling TestU01 functions. In the unlikely event you call
one the wrapped function from one of the TestU01 callback, this will lead to a 
deadlock.


## License

This program is free software: you can redistribute it and/or modify it under 
the terms of the GNU General Public License as published by the 
Free Software Foundation, either version 3 of the License, or (at your option)
any later version.

This program is distributed in the hope that it will be useful, but WITHOUT ANY 
WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A 
PARTICULAR PURPOSE.  See the GNU General Public License for more details.

## Thanks

In addition to TestU01 authors, I would like to thank some other projects that 
I used when implementing this wrapper:
 * [rust-bindgen](https://github.com/crabtw/rust-bindgen)
