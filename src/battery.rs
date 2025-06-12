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

//! This module covers the bbattery part of TestU01. It allows you to apply
//! predefined batteries of
//! tests to a generator.
//! To have more detail about each test and the meaning of each parameters, see
//! the TestU01 manual.

use std::ffi::{CStr, CString};

use crate::GLOBAL_LOCK;
use testu01_sys::sentrop_Res;

use indexmap::IndexMap;

macro_rules! wrap {
    ($name:ident, $wrapped:ident) => (
        wrap!($name, $wrapped, );
    );
   ($name:ident, $wrapped:ident, $($arg_name:ident: $arg_type:ty),*) => (
        pub fn $name<T: $crate::unif01::WithRawUnif01Gen>(gen: &mut T, $($arg_name: $arg_type),*) -> BatteryResults {
            let _g = $crate::GLOBAL_LOCK.lock().unwrap();
            let wrapper = |gen| {
                // Reset the number of tests to 0
                unsafe { testu01_sys::bbattery_NTests = 0 };
                unsafe { testu01_sys::$wrapped(gen $(, $arg_name)*) };
            };
            gen.with_raw(wrapper);
            get_results()
        }
    );
}

macro_rules! wrap_file {
    ($name:ident, $wrapped:ident) => (
        wrap_file!($name, $wrapped, );
    );
    ($name:ident, $wrapped:ident, $($arg_name:ident: $arg_type:ty),*) => (
        pub fn $name(path: &str, $($arg_name: $arg_type),*) -> BatteryResults {
            let path: CString = CString::new(path).unwrap();
            let _g = $crate::GLOBAL_LOCK.lock().unwrap();
            // Reset the number of tests to 0
            unsafe { testu01_sys::bbattery_NTests = 0 };
            unsafe { testu01_sys::$wrapped(path.as_c_str().as_ptr() as *mut _ $(, $arg_name)*) };
            get_results()
        }
    );
}

macro_rules! wrap_repeat {
    ($name:ident, $wrapped:ident, $num_repeats:expr ) => (
        wrap_repeat!($name, $wrapped, $num_repeats, );
    );
    ($name:ident, $wrapped:ident, $num_repeats:expr, $($arg_name:ident: $arg_type:ty),* $(; $xarg_name:ident: $xarg_type:ty )? ) => (
        pub fn $name<T: $crate::unif01::WithRawUnif01Gen>(gen: &mut T $(, $arg_name: $arg_type)*, rep: &mut [::libc::c_int] $(,$xarg_name: $xarg_type)? ) -> BatteryResults {
            assert!(rep.len() > $num_repeats, "rep.len() must be at least {}", $num_repeats+1);
            let _g = GLOBAL_LOCK.lock().unwrap();

            // Reset the number of tests to 0
            unsafe { testu01_sys::bbattery_NTests = 0 };

            let wrapper = |gen| {
                unsafe { testu01_sys::$wrapped(gen  $(, $arg_name)*, rep.as_ptr() as *mut _ $(, $xarg_name)? ) };
            };
            gen.with_raw(wrapper);
            get_results()
        }
    );
}

wrap!(crush, bbattery_Crush);
wrap!(small_crush, bbattery_SmallCrush);
wrap!(big_crush, bbattery_BigCrush);
wrap!(rabbit, bbattery_Rabbit, nb: libc::c_double);
wrap!(alphabit, bbattery_Alphabit, nb: libc::c_double, r: libc::c_int, s: libc::c_int);
wrap!(block_alphabit, bbattery_BlockAlphabit, nb: libc::c_double, r: libc::c_int, s: libc::c_int);
wrap!(pseudo_diehard, bbattery_pseudoDIEHARD);
wrap!(fips_140_2, bbattery_FIPS_140_2);
wrap_file!(rabbit_file, bbattery_RabbitFile, nb: libc::c_double);
wrap_file!(alphabit_file, bbattery_AlphabitFile, nb: libc::c_double);
wrap_file!(block_alphabit_file, bbattery_BlockAlphabitFile, nb: libc::c_double);
wrap_file!(fips_140_2_file, bbattery_FIPS_140_2File);
wrap_repeat!(repeat_crush, bbattery_RepeatCrush, 96);
wrap_repeat!(repeat_big_crush, bbattery_RepeatBigCrush, 106);
wrap_repeat!(repeat_rabbit, bbattery_RepeatRabbit, 26, nb: libc::c_double);
wrap_repeat!(repeat_alphabit, bbattery_RepeatAlphabit, 9, nb: libc::c_double, r: libc::c_int, s: libc::c_int);
wrap_repeat!(repeat_block_alphabit, bbattery_RepeatBlockAlphabit, 9, nb: libc::c_double, r: libc::c_int, s: libc::c_int ; w: libc::c_int);
wrap_repeat!(repeat_small_crush, bbattery_RepeatSmallCrush, 10);

/**
 *
 *  Applies\index{Test!EntropyDiscOver} an entropy-based test
  described in \cite{rLEC96e}, similar
  to {\tt sentrop\_EntropyDisc}, but with overlap of the blocks.
  It constructs a sequence of $n$ bits, by taking $s$ bits from each
  of $n/s$ output values, puts these $n$ bits on a circle,
  and examines all $n$ blocks of $L$ successive bits on this circle.
  The test computes the empirical entropy, defined by
   $$ T = -\sum_{i = 0}^{k-1} X_i \log_2 X_i, $$
  where the $X_i$ are the observed frequencies of the $L$-bit strings.
  This test is equivalent to {\tt smultin\_MultinomialBitsOver} with the
  power divergence test statistic, using $\delta=0$ only.
%  Note: This is different from {\tt smultin\_MultinomialOver}
%  unless $s=1$, because here the overlapping is for sequences of {\em bits}.

  For $N>1$, the function also tests the empirical correlation
  between pairs of successive values of $T$, as well as the average
  of these values.  This average is compared with the exact expectation
  in the cases where it is known.
  Restrictions:  r <= 31, s < 31, n <= 31, L <= n/2, n mod s = 0, and N >> n.
 */
pub fn entropy_disc<T: crate::unif01::WithRawUnif01Gen>(
    gen: &mut T,
    pairs: libc::c_long,
    num_blocks: libc::c_long,
    drop_bits: libc::c_int,
    take_bits: libc::c_int,
    block_bits: libc::c_int,
) {
    // assert!(
    //     drop_bits <= 31
    //         && take_bits < 31
    //         && num_blocks <= 31
    //         && block_bits <= (num_blocks / 2) as libc::c_int
    //         && (num_blocks as libc::c_int % take_bits as libc::c_int == 0)
    //         && pairs > num_blocks
    // );
    let _g = GLOBAL_LOCK.lock().unwrap();

    // let res: *mut sentrop_Res = unsafe { testu01_sys::sentrop_CreateRes() };
    // assert!(!res.is_null());

    let res: *mut sentrop_Res = std::ptr::null_mut() as _;

    let wrapper = |gen| unsafe {
        testu01_sys::sentrop_EntropyDisc(
            gen, res, pairs, num_blocks, drop_bits, take_bits, block_bits,
        );
    };
    gen.with_raw(wrapper);
}

#[derive(Debug, Clone)]
pub struct BatteryResults {
    pub p_values: IndexMap<String, f64>,
    pub passed: IndexMap<String, bool>,
    // pub test_numbers: HashMap<String, u32>,
}

impl BatteryResults {
    pub fn with_capacity(capacity: usize) -> BatteryResults {
        BatteryResults {
            p_values: IndexMap::with_capacity(capacity),
            passed: IndexMap::with_capacity(capacity),
        }
    }
}

/// Gets the p-values of the tests of the last battery applied.
fn get_results() -> BatteryResults {
    let len = unsafe { testu01_sys::bbattery_NTests };
    assert!(len >= 0);
    let len = len as usize;
    let mut results = BatteryResults::with_capacity(len);
    for i in 0..len {
        let ptr = unsafe { testu01_sys::bbattery_TestNames[i] };
        let name = if ptr.is_null() {
            "".to_string()
        } else {
            // SAFETY: we already checked that `ptr` is not NULL
            let name_bytes = unsafe { CStr::from_ptr(ptr) }.to_bytes();
            std::str::from_utf8(name_bytes).unwrap_or("").to_string()
        };
        let pvalue = unsafe { testu01_sys::bbattery_pVal[i] };
        if pvalue >= 0.0 {
            results.p_values.insert(name.clone(), pvalue);
        }
        let passed: i32 = unsafe { testu01_sys::bbattery_pass[i] };
        if passed >= 0 {
            results.passed.insert(name, passed == 1);
        }
    }
    results
}
