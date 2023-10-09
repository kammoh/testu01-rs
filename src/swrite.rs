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

//! This module covers the swrite part of TestU01. It allows you to configure
//! what is printed when
//! tests are run.

use crate::GLOBAL_LOCK;
use std::ffi::CString;

macro_rules! wrap {
    ($name:ident, $wrapped:path) => {
        pub fn $name(value: bool) {
            let _g = GLOBAL_LOCK.lock().unwrap();
            unsafe { $wrapped = value as testu01_sys::lebool };
        }
    };
}

wrap!(set_basic, testu01_sys::swrite_Basic);
wrap!(set_parameters, testu01_sys::swrite_Parameters);
wrap!(set_collectors, testu01_sys::swrite_Collectors);
wrap!(set_classes, testu01_sys::swrite_Classes);
wrap!(set_counters, testu01_sys::swrite_Counters);
wrap!(set_host, testu01_sys::swrite_Host);

pub fn set_experiment_name(name: &CString) {
    let _g = GLOBAL_LOCK.lock().unwrap();
    unsafe { testu01_sys::swrite_SetExperimentName(name.as_ptr() as *mut _) }
}
