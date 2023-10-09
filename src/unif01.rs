// A rust wrapper to a small subset of TestU01
// (http://simul.iro.umontreal.ca/testu01/tu01.html).
// Copyright (C) 2015  Loïc Damien
// Copyright (C) 2023  Kamyar Mohajerani
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

//! This module allows you to define generators than can be tested by TestU01.
//! It doesn't covers the rest of the unif01 part of TestU01.

use rand::Rng;
use std::ffi::CString;
use std::fmt;
use std::ptr::null_mut;

pub const UNIF01_NORM32: f64 = (1u64 << 32) as f64;
pub const UNIF01_INV32: f64 = 1.0 / UNIF01_NORM32;

/// Any type than can be converted to ffi::raw_unif01_Gen should implement this trait
pub trait WithRawUnif01Gen {
    fn with_raw<R, F>(&mut self, f: F) -> R
    where
        F: FnOnce(*mut testu01_sys::unif01_Gen) -> R;
}

pub trait Unif01Methods {
    /// Return a floating point number in the range [0, 1).
    fn get_u01(&mut self) -> f64;
    /// Return a block of random bits.
    /// If the generator produce less than 32 bits, the relevant ones must be
    /// the most significant ones.
    fn get_bits(&mut self) -> u32;
    /// Output the internal state of the generator.
    fn write(&mut self);
}

extern "C" fn get_u01_wrapper<T: Unif01Methods>(
    _param: *mut ::libc::c_void,
    gen: *mut ::libc::c_void,
) -> ::libc::c_double {
    let gen: &mut T = unsafe { &mut *(gen as *mut T) };
    gen.get_u01() as ::libc::c_double
}

extern "C" fn get_bits_wrapper<T: Unif01Methods>(
    _param: *mut ::libc::c_void,
    gen: *mut ::libc::c_void,
) -> ::libc::c_ulong {
    let gen: &mut T = unsafe { &mut *(gen as *mut T) };
    gen.get_bits() as ::libc::c_ulong
}

extern "C" fn write_wrapper<T: Unif01Methods>(gen: *mut ::libc::c_void) {
    let gen: &mut T = unsafe { &mut *(gen as *mut T) };
    gen.write();
}

pub struct Unif01Gen<T> {
    state: T,
    name: CString,
}

impl<T> Unif01Gen<T> {
    pub fn new(state: T, name: &str) -> Unif01Gen<T> {
        Unif01Gen {
            state,
            name: CString::new(name).unwrap(),
        }
    }
}

impl<T: Unif01Methods> WithRawUnif01Gen for Unif01Gen<T> {
    fn with_raw<R, F>(&mut self, f: F) -> R
    where
        F: FnOnce(*mut testu01_sys::unif01_Gen) -> R,
    {
        let mut raw = testu01_sys::unif01_Gen {
            state: &mut self.state as *mut T as *mut ::libc::c_void,
            param: null_mut(),
            name: self.name.as_ptr() as *mut _,
            GetU01: Some(get_u01_wrapper::<T>),
            GetBits: Some(get_bits_wrapper::<T>),
            Write: Some(write_wrapper::<T>),
        };
        f(&mut raw)
    }
}

impl<T> Unif01Methods for T
where
    T: Rng + fmt::Debug,
{
    fn get_u01(&mut self) -> f64 {
        self.next_u32() as f64 * UNIF01_INV32
    }

    fn get_bits(&mut self) -> u32 {
        self.next_u32()
    }

    fn write(&mut self) {
        println!("{:?}", self);
    }
}

pub struct Unif01Pair<T, F>(pub T, pub F);

impl<T, F> Unif01Methods for Unif01Pair<T, F>
where
    T: Rng,
    F: FnMut(&mut T),
{
    fn get_u01(&mut self) -> f64 {
        self.0.next_u32() as f64 * UNIF01_INV32
    }

    fn get_bits(&mut self) -> u32 {
        self.0.next_u32()
    }

    fn write(&mut self) {
        self.1(&mut self.0)
    }
}
