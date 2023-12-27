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

//! This module doesn't wrap any part on TestU01. It provides decorators to
//! help you test
//! your random number generators more thoroughly.

use rand::{RngCore, Error};


/// A generator that reverse the order of the bits produced by another generator.
///
/// It seems that TestU01 is biased toward the most significant bits. Reversing the order of the
/// bits allow you to detect more flaws in generators.
#[derive(Debug)]
pub struct ReverseBits<T: RngCore> {
    pub rng: T,
}

impl<T: RngCore> ReverseBits<T> {
    pub fn new(rng: T) -> ReverseBits<T> {
        ReverseBits { rng: rng }
    }
}

// Technique from
// http://graphics.stanford.edu/~seander/bithacks.html#ReverseParallel
#[inline]
fn reverse_bits32(bits: u32) -> u32 {
    let bits = ((bits >> 1) & 0x55555555) | ((bits & 0x55555555) << 1);
    let bits = ((bits >> 2) & 0x33333333) | ((bits & 0x33333333) << 2);
    let bits = ((bits >> 4) & 0x0F0F0F0F) | ((bits & 0x0F0F0F0F) << 4);
    let bits = ((bits >> 8) & 0x00FF00FF) | ((bits & 0x00FF00FF) << 8);
    let bits = (bits >> 16) | (bits << 16);
    bits
}

#[inline]
fn reverse_bits64(bits: u64) -> u64 {
    let bits = ((bits >> 1) & 0x5555555555555555) | ((bits & 0x5555555555555555) << 1);
    let bits = ((bits >> 2) & 0x3333333333333333) | ((bits & 0x3333333333333333) << 2);
    let bits = ((bits >> 4) & 0x0F0F0F0F0F0F0F0F) | ((bits & 0x0F0F0F0F0F0F0F0F) << 4);
    let bits = ((bits >> 8) & 0x00FF00FF00FF00FF) | ((bits & 0x00FF00FF00FF00FF) << 8);
    let bits = ((bits >> 16) & 0x0000FFFF0000FFFF) | ((bits & 0x0000FFFF0000FFFF) << 16);
    let bits = (bits >> 32) | (bits << 32);
    bits
}

impl<T: RngCore> RngCore for ReverseBits<T> {
    #[inline]
    fn next_u32(&mut self) -> u32 {
        reverse_bits32(self.rng.next_u32())
    }

    #[inline]
    fn next_u64(&mut self) -> u64 {
        reverse_bits64(self.rng.next_u64())
    }

    fn fill_bytes(&mut self, dest: &mut [u8]) {
        self.rng.fill_bytes(dest);
        dest.reverse();
        dest.iter_mut().for_each(|x| *x = x.reverse_bits());
    }

    fn try_fill_bytes(&mut self, dest: &mut [u8]) -> Result<(), Error> {
        self.rng.try_fill_bytes(dest)?;
        dest.reverse();
        dest.iter_mut().for_each(|x| *x = x.reverse_bits());
        Ok(())
    }
}

/// A generator that successively emit the upper the upper and lower half of the values produced
/// by the next_u64 methods of another generator when next_u32 is called.
///
/// TestU01 is a 32 bits test suite. You can use this decorator to transform a 64 bits generator
/// to a 32 bit generator that use all 64 bits of it.
#[derive(Debug)]
pub struct Rng64To32<T: RngCore> {
    pub rng: T,
    lower_half: Option<u32>,
}

impl<T: RngCore> Rng64To32<T> {
    pub fn new(rng: T) -> Rng64To32<T> {
        Rng64To32 {
            rng: rng,
            lower_half: None,
        }
    }
}

impl<T: RngCore> RngCore for Rng64To32<T> {
    #[inline]
    fn next_u32(&mut self) -> u32 {
        if let Some(n) = self.lower_half {
            self.lower_half = None;
            n
        } else {
            let n = self.rng.next_u64();
            self.lower_half = Some(n as u32);
            (n >> 32) as u32
        }
    }

    fn next_u64(&mut self) -> u64 {
        self.rng.next_u64()
    }

    fn fill_bytes(&mut self, dest: &mut [u8]) {
        self.rng.fill_bytes(dest);
    }

    fn try_fill_bytes(&mut self, dest: &mut [u8]) -> Result<(), Error> {
        self.rng.try_fill_bytes(dest)
    }
}

#[cfg(test)]
mod test {
    #[test]
    fn test_reverse32() {
        let foo = [(0x80000000, 0x00000001),
                   (0x6b7c265b, 0xda643ed6),
                   (0xda643ed6, 0x6b7c265b),
                   (0xbc96c03d, 0xbc03693d)];
        for &(bits, expected) in foo.iter() {
            assert_eq!(super::reverse_bits32(bits), expected);
        }
    }

    #[test]
    fn test_reverse64() {
        let foo = [(0x8000000000000000, 0x0000000000000001),
                   (0x08f58f42407f4819, 0x9812fe0242f1af10),
                   (0xda643ed600000000, 0x000000006b7c265b),
                   (0x00000000bc96c03d, 0xbc03693d00000000)];
        for &(bits, expected) in foo.iter() {
            assert_eq!(super::reverse_bits64(bits), expected);
        }
    }
}
