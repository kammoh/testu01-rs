use std::ffi::{c_str_to_bytes, CString};
use std::str;

mod ffi {
    #[link(name = "testu01")]
    extern "C" {
        pub static mut bbattery_NTests: ::libc::c_int;
        pub static mut bbattery_pVal: [::libc::c_double; 0];
        pub static mut bbattery_TestNames: [*const ::libc::c_char; 0];
    }

    #[link(name = "testu01")]
    extern "C" {
        pub fn bbattery_SmallCrush(gen: *mut ::unif01::ffi::raw_unif01_Gen);
        pub fn bbattery_SmallCrushFile(filename: *const ::libc::c_char);
        pub fn bbattery_RepeatSmallCrush(gen: *mut ::unif01::ffi::raw_unif01_Gen, rep: *const ::libc::c_int);
        pub fn bbattery_Crush(gen: *mut ::unif01::ffi::raw_unif01_Gen);
        pub fn bbattery_RepeatCrush(gen: *mut ::unif01::ffi::raw_unif01_Gen, rep: *const ::libc::c_int);
        pub fn bbattery_BigCrush(gen: *mut ::unif01::ffi::raw_unif01_Gen);
        pub fn bbattery_RepeatBigCrush(gen: *mut ::unif01::ffi::raw_unif01_Gen, rep: *const ::libc::c_int);
        pub fn bbattery_Rabbit(gen: *mut ::unif01::ffi::raw_unif01_Gen, nb: ::libc::c_double);
        pub fn bbattery_RabbitFile(filename: *const ::libc::c_char, nb: ::libc::c_double);
        pub fn bbattery_RepeatRabbit(gen: *mut ::unif01::ffi::raw_unif01_Gen, nb: ::libc::c_double, rep: *const ::libc::c_int);
        pub fn bbattery_Alphabit(gen: *mut ::unif01::ffi::raw_unif01_Gen, nb: ::libc::c_double, r: ::libc::c_int, s: ::libc::c_int);
        pub fn bbattery_AlphabitFile(filename: *const ::libc::c_char, nb: ::libc::c_double);
        pub fn bbattery_RepeatAlphabit(gen: *mut ::unif01::ffi::raw_unif01_Gen, nb: ::libc::c_double, r: ::libc::c_int, s: ::libc::c_int, rep: *const ::libc::c_int);
        pub fn bbattery_BlockAlphabit(gen: *mut ::unif01::ffi::raw_unif01_Gen, nb: ::libc::c_double, r: ::libc::c_int, s: ::libc::c_int);
        pub fn bbattery_BlockAlphabitFile(filename: *const ::libc::c_char, nb: ::libc::c_double);
        pub fn bbattery_RepeatBlockAlphabit(gen: *mut ::unif01::ffi::raw_unif01_Gen, nb: ::libc::c_double, r: ::libc::c_int, s: ::libc::c_int, rep: *const ::libc::c_int, w: ::libc::c_int);
        pub fn bbattery_pseudoDIEHARD(gen: *mut ::unif01::ffi::raw_unif01_Gen);
        pub fn bbattery_FIPS_140_2(gen: *mut ::unif01::ffi::raw_unif01_Gen);
        pub fn bbattery_FIPS_140_2File(filename: *const ::libc::c_char);
    }

}

macro_rules! wrap {
    ($name:ident, $wrapped:path) => (
        wrap!($name, $wrapped, );
    );
   ($name:ident, $wrapped:path, $($arg_name:ident: $arg_type:ty),*) => (
        pub fn $name<T: ::unif01::WithRawUnif01Gen>(gen: &mut T, $($arg_name: $arg_type),*) { 
            let wrapper = |&: gen: &mut ::unif01::ffi::raw_unif01_Gen| {
                let _g = ::GLOBAL_LOCK.lock().unwrap();
                unsafe { $wrapped(gen $(, $arg_name)*) };
            };
            gen.with_raw(wrapper);
        }
    );
}

macro_rules! wrap_file {
    ($name:ident, $wrapped:path) => (
        wrap_file!($name, $wrapped, );
    );
    ($name:ident, $wrapped:path, $($arg_name:ident: $arg_type:ty),*) => (
        pub fn $name<T: GenericPath>(path: &mut T, $($arg_name: $arg_type),*) { 
            let c_path = CString::from_slice(path.as_vec());
            let _g = ::GLOBAL_LOCK.lock().unwrap();
            unsafe { $wrapped(c_path.as_ptr() $(, $arg_name)*) };
        }
    );
}

macro_rules! wrap_rep {
    ($name:ident, $wrapped:path, $rep:expr) => (
        wrap_rep!($name, $wrapped, $rep, );
    );
    ($name:ident, $wrapped:path, $rep:expr, $($arg_name:ident: $arg_type:ty),*) => (
        pub fn $name<T: ::unif01::WithRawUnif01Gen>(gen: &mut T $(, $arg_name: $arg_type)*, rep: &[::libc::c_int]) { 
            let wrapper = |&: gen: &mut ::unif01::ffi::raw_unif01_Gen| {
                let _g = ::GLOBAL_LOCK.lock().unwrap();
                unsafe { $wrapped(gen  $(, $arg_name)*, rep.as_ptr()) };
            };
            assert!(rep.len() == $rep+1);
            gen.with_raw(wrapper);
        }
    );
}

wrap!(small_crush, ffi::bbattery_SmallCrush);
wrap_file!(small_crush_file, ffi::bbattery_SmallCrushFile);
wrap_rep!(repeat_small_crush, ffi::bbattery_RepeatSmallCrush, 10);
wrap!(crush, ffi::bbattery_Crush);
wrap_rep!(repeat_crush, ffi::bbattery_RepeatCrush, 96);
wrap!(big_crush, ffi::bbattery_BigCrush);
wrap_rep!(repeat_big_crush, ffi::bbattery_RepeatBigCrush, 106);
wrap!(rabbit, ffi::bbattery_Rabbit, nb: ::libc::c_double);
wrap_file!(rabbit_file, ffi::bbattery_RabbitFile, nb: ::libc::c_double);
wrap_rep!(repeat_rabbit, ffi::bbattery_RepeatRabbit, 26, nb: ::libc::c_double);
wrap!(alphabit, ffi::bbattery_Alphabit, nb: ::libc::c_double, r: ::libc::c_int, s: ::libc::c_int);
wrap_file!(alphabit_file, ffi::bbattery_AlphabitFile, nb: ::libc::c_double);
wrap_rep!(repeat_alphabit, ffi::bbattery_RepeatAlphabit, 9, nb: ::libc::c_double, r: ::libc::c_int, s: ::libc::c_int);
wrap!(block_alphabit, ffi::bbattery_BlockAlphabit, nb: ::libc::c_double, r: ::libc::c_int, s: ::libc::c_int);
wrap_file!(block_alphabit_file, ffi::bbattery_BlockAlphabitFile, nb: ::libc::c_double);
wrap!(pseudo_diehard, ffi::bbattery_pseudoDIEHARD);
wrap!(fips_140_2, ffi::bbattery_FIPS_140_2);
wrap_file!(fips_140_2_file, ffi::bbattery_FIPS_140_2File);

// Not using wrap_rep! because bbattery_RepeatBlockAlphabit want another argument after the rep argument.
pub fn repeat_block_alphabit<T: ::unif01::WithRawUnif01Gen>(gen: &mut T,  nb: ::libc::c_double, r: ::libc::c_int, s: ::libc::c_int, rep: &[::libc::c_int], w: ::libc::c_int) {
    let wrapper = |&: gen: &mut ::unif01::ffi::raw_unif01_Gen| {
        let _g = ::GLOBAL_LOCK.lock().unwrap();
        unsafe { ffi::bbattery_RepeatBlockAlphabit(gen, nb, r, s, rep.as_ptr(), w) };
    };
    assert!(rep.len() == 9+1);
    gen.with_raw(wrapper);
}

pub fn get_pvalues() -> Vec<(String, f64)> {
    let _g = ::GLOBAL_LOCK.lock().unwrap();
    let len = unsafe { ffi::bbattery_NTests };
    assert!(len >= 0);
    let len = len as usize;
    let mut pvalues = Vec::with_capacity(len);
    for i in 0..len {
        let pvalue = unsafe{ *ffi::bbattery_pVal.get_unchecked(i) };
        let name = unsafe { c_str_to_bytes(ffi::bbattery_TestNames.get_unchecked(i)) };
        let name = str::from_utf8(name).unwrap_or("").to_string();
        pvalues.push((name, pvalue))
    }
    return pvalues;
}
