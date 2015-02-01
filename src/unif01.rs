use std::ffi::CString;
use std::fmt;
use std::ptr::null_mut;
use std::rand::Rng;

pub mod ffi {
   #[allow(non_camel_case_types)]
   #[allow(non_snake_case)]
   #[repr(C)]
   pub struct raw_unif01_Gen {
        pub state: *mut ::libc::c_void,
        pub param: *mut ::libc::c_void,
        pub name: *const ::libc::c_char,
        pub GetU01: Option<extern "C" fn(param: *mut ::libc::c_void, state: *mut ::libc::c_void) -> ::libc::c_double>,
        pub GetBits: Option<extern "C" fn(param: *mut ::libc::c_void, state: *mut ::libc::c_void) -> ::libc::c_ulong>,
        pub Write: Option<extern "C" fn(state: *mut ::libc::c_void)>,
    }
    
    impl Copy for raw_unif01_Gen {}
}

pub trait WithRawUnif01Gen {
    fn with_raw<R, F>(&mut self, f: F) -> R where F: FnOnce(&mut ffi::raw_unif01_Gen) -> R;
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

extern "C" fn get_u01_wrapper<T: Unif01Methods>(_param: *mut ::libc::c_void, gen: *mut ::libc::c_void) -> ::libc::c_double {
    let gen: &mut T = unsafe { &mut *(gen as *mut T) };
    gen.get_u01() as ::libc::c_double
}

extern "C" fn get_bits_wrapper<T: Unif01Methods>(_param: *mut ::libc::c_void, gen: *mut ::libc::c_void) -> ::libc::c_ulong {
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

    pub fn new(state: T, name: CString) -> Unif01Gen<T> {
         Unif01Gen {state: state, name: name}
    }
}

impl<T: Unif01Methods> WithRawUnif01Gen for Unif01Gen<T> {
    fn with_raw<R, F>(&mut self, f: F) -> R 
        where F: FnOnce(&mut ffi::raw_unif01_Gen) -> R {
        let mut raw = ffi::raw_unif01_Gen {
            state: &mut self.state as *mut T as *mut ::libc::c_void,
            param: null_mut(),
            name: self.name.as_ptr(),
            GetU01: Some(get_u01_wrapper::<T>),
            GetBits: Some(get_bits_wrapper::<T>),
            Write: Some(write_wrapper::<T>),
        };
        f(&mut raw)
    }
}


impl<T> Unif01Methods for T where T: Rng+fmt::Debug {
    fn get_u01(&mut self) -> f64 {
        self.gen::<f64>()
    }
    
    fn get_bits(&mut self) -> u32 {
        self.gen::<u32>()
    }
    
    fn write(&mut self) {
        println!("{:?}", self);
    }
}

impl<T, F> Unif01Methods for (T, F) where T: Rng, F: FnMut(&mut T) {
    fn get_u01(&mut self) -> f64 {
        self.0.gen::<f64>()
    }
    
    fn get_bits(&mut self) -> u32 {
        self.0.gen::<u32>()
    }
    
    fn write(&mut self) {
        self.1(&mut self.0)
    }
}
