use std::ffi::CString;

mod ffi {
    #[allow(non_camel_case_types)]
    type lebool = ::libc::c_int;
    
    #[link(name = "testu01")]
    extern "C" {
        pub static mut swrite_Basic: lebool;
        pub static mut swrite_Parameters: lebool;
        pub static mut swrite_Collectors: lebool;
        pub static mut swrite_Classes: lebool;
        pub static mut swrite_Counters: lebool;
        pub static mut swrite_Host: lebool;
        //pub static mut swrite_ExperimentName: *mut ::libc::c_char;
    }
    #[link(name = "testu01")]
    extern "C" {
        pub fn swrite_SetExperimentName(Name: *const ::libc::c_char) -> ();
    }
}

macro_rules! wrap {
    ($name:ident, $wrapped:path) => (
        pub fn $name(value: bool) { 
            let _g = ::GLOBAL_LOCK.lock().unwrap();
            unsafe { $wrapped = value as ::libc::c_int };
        }
    );
}

wrap!(set_basic, ffi::swrite_Basic);
wrap!(set_parameters, ffi::swrite_Parameters);
wrap!(set_collectors, ffi::swrite_Collectors);
wrap!(set_classes, ffi::swrite_Classes);
wrap!(set_counters, ffi::swrite_Counters);
wrap!(set_host, ffi::swrite_Host);

pub fn set_experiment_name(name: &CString) {
    let _g = ::GLOBAL_LOCK.lock().unwrap();
    unsafe { ffi::swrite_SetExperimentName(name.as_ptr()) };
}
