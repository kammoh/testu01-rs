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

pub mod xorshift;

use rand::{rngs::ThreadRng, Rng};
use rand_core::SeedableRng;
use testu01::{swrite, battery::{small_crush, fips_140_2, crush}};
use testu01::unif01::{Unif01Gen, Unif01Pair};
use xorshift::XorShiftRng;

// XorShiftRng doesn't implement the Debug trait but we want to print its
// internal state.
// This is an ugly hack to access his private members and print them.
fn write(gen: &mut XorShiftRng) {
    println!("{}", gen);
}

fn main() {
    swrite::set_experiment_name("Test of rust weak rng with small crush");
    swrite::set_host(false); // Disable the printing of the hostname in the results
    swrite::set_basic(true); // basic = verbose!

    let mut thread_rng = ThreadRng::default();

    let seed = thread_rng.gen::<[u8; 16]>(); // The seed of the generator
    let xor_shift_rng: XorShiftRng = XorShiftRng::from_seed(seed); // The generator that will be tested.

    // Build an object than can  be converted to something that TestU01 can test:
    let mut xorshift_unif01 = Unif01Gen::new(Unif01Pair(xor_shift_rng, write), "weak_rng");

    // Apply the small crush battery to it:
    small_crush(&mut xorshift_unif01);


    // Print the p-values for the differents test of the battery:
    let p_values = testu01::battery::get_pvalues();
    println!("Small Crush P-values:\n----------------------------------");

    for (key, value) in &p_values {
        println!("{:25} {:.6}", key, value);
    }


    // fips_140_2(&mut xorshift_unif01);
    // let p_values = testu01::battery::get_pvalues();
    // println!("NIST-140-2 P-values:\n----------------------------------");

    // for (key, value) in &p_values {
    //     println!("{:25} {:.6}", key, value);
    // }
}
