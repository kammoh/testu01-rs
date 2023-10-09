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

//! This crate is a wrapper around a small subset of TestU01.

#![cfg_attr(feature = "i128_support", feature(i128_type))]

#[macro_use]
extern crate lazy_static;

use std::sync::Mutex;

lazy_static! {
    /// Lot of TestU01 is inherently non thread-safe, updating/reading global variables without
    /// synchronization. This lock is here to protect access to all TestU01 global variables.
    static ref GLOBAL_LOCK: Mutex<()> = Mutex::new(());
}

pub mod battery;
pub mod decorators;
pub mod swrite;
pub mod unif01;
