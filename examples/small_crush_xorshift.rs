#![feature(rand)]
#![feature(std_misc)]

extern crate testu01;

use std::ffi::CString;
use std::rand;

use testu01::unif01::Unif01Gen;
use testu01::swrite;

// XorShiftRng doesn't implement the Debug trait but we want to print its internal state.
// This is an ugly hack to access his private members and print them.
fn write(gen: &mut rand::XorShiftRng) {
    let gen: &(u32, u32, u32, u32) = unsafe { std::mem::transmute(gen) };
    println!("x: 0x{:x}, y: 0x{:x}, z: 0x{:x}, w: 0x{:x}", gen.0, gen.1, gen.2, gen.3)
}

fn main() {
    let experiment_name = "Test of rust weak rng with small crush";
    swrite::set_experiment_name(&CString::from_slice(experiment_name.as_bytes()));
    swrite::set_host(false); // Disable the printing of the hostname in the results

    let name = "weak_rng";
    let c_name = CString::from_slice(name.as_bytes());

    let rng = rand::XorShiftRng::new_unseeded(); // The generator tht will be tested.

    // Build an object than can  be converted to something that TestU01 can test:
    let mut xorshift_unif01 = Unif01Gen::new((rng, write), c_name); 
    
    // Apply the small crush battery to it:
    testu01::battery::small_crush(&mut xorshift_unif01);
    
    // Print the p-values for the differents test of the battery:
    println!("P-values: {:?}", testu01::battery::get_pvalues()); 
}
