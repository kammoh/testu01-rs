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

//! This module covers the swrite part of TestU01. It allows you to configure
//! what is printed when
//! tests are run.

use crate::GLOBAL_LOCK;
use std::ffi::CString;

mod ffi {
    #[allow(non_camel_case_types)]
    pub type lebool = ::libc::c_int;

    #[link(name = "testu01")]
    extern "C" {
        pub static mut swrite_Basic: lebool;
        pub static mut swrite_Parameters: lebool;
        pub static mut swrite_Collectors: lebool;
        pub static mut swrite_Classes: lebool;
        pub static mut swrite_Counters: lebool;
        pub static mut swrite_Host: lebool;
        // pub static mut swrite_ExperimentName: *mut ::libc::c_char;
    }
    #[link(name = "testu01")]
    extern "C" {
        pub fn swrite_SetExperimentName(Name: *const ::libc::c_char);
    }
}

macro_rules! wrap {
    ($name:ident, $wrapped:path) => {
        pub fn $name(value: bool) {
            let _g = GLOBAL_LOCK.lock().unwrap();
            unsafe { $wrapped = value as ffi::lebool };
        }
    };
}

wrap!(set_basic, ffi::swrite_Basic);
wrap!(set_parameters, ffi::swrite_Parameters);
wrap!(set_collectors, ffi::swrite_Collectors);
wrap!(set_classes, ffi::swrite_Classes);
wrap!(set_counters, ffi::swrite_Counters);
wrap!(set_host, ffi::swrite_Host);

pub fn set_experiment_name(name: &CString) {
    let _g = GLOBAL_LOCK.lock().unwrap();
    unsafe { ffi::swrite_SetExperimentName(name.as_ptr()) }
}
