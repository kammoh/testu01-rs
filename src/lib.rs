//! This crate is a wrapper around a small subset of TestU01.

#![feature(core)]
#![feature(libc)]
#![feature(rand)]
#![feature(std_misc)]

extern crate libc;

use std::sync::{StaticMutex, MUTEX_INIT};

/// Lot of TestU01 is inherently non thread-safe, updating/reading global variables without synchronization.
/// This lock is here to protect access to all TestU01 global variables.
static GLOBAL_LOCK: StaticMutex = MUTEX_INIT;

pub mod decorators;
pub mod battery;
pub mod unif01;
pub mod swrite;
