// A rust wrapper to a small subset of TestU01
// (http://simul.iro.umontreal.ca/testu01/tu01.html).
// Copyright (C) 2015  Loïc Damien
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with this program.  If not, see <http://www.gnu.org/licenses/>.

extern crate testu01;
extern crate rand;
extern crate rand_xorshift;

use std::ffi::CString;
use rand::SeedableRng;
use rand_xorshift::XorShiftRng;

use testu01::unif01::{Unif01Gen, Unif01Pair};
use testu01::swrite;

// XorShiftRng doesn't implement the Debug trait but we want to print its
// internal state.
// This is an ugly hack to access his private members and print them.
fn write(gen: &mut XorShiftRng) {
    let gen: &(u32, u32, u32, u32) = unsafe { std::mem::transmute(gen) };
    println!("x: 0x{:x}, y: 0x{:x}, z: 0x{:x}, w: 0x{:x}",
             gen.0,
             gen.1,
             gen.2,
             gen.3)
}

fn main() {
    let experiment_name = "Test of rust weak rng with small crush";
    swrite::set_experiment_name(&CString::new(experiment_name).unwrap());
    swrite::set_host(false); // Disable the printing of the hostname in the results

    let name = "weak_rng";
    let c_name = CString::new(name).unwrap();

    let rng = XorShiftRng::from_seed([0; 16]); // The generator that will be tested.

    // Build an object than can  be converted to something that TestU01 can test:
    let mut xorshift_unif01 = Unif01Gen::new(Unif01Pair(rng, write), c_name);

    // Apply the small crush battery to it:
    testu01::battery::small_crush(&mut xorshift_unif01);

    // Print the p-values for the differents test of the battery:
    println!("P-values: {:?}", testu01::battery::get_pvalues());
}
