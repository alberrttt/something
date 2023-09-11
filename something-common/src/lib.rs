#![feature(try_trait_v2)]
#[macro_export]
macro_rules! devprintln {
    ($($arg:tt)*) => {
        if cfg!(debug_assertions) {
            print!(concat!("[",file!(), ":", line!(), "]: "));
            println!($($arg)*);

        }
    }
}

pub mod msg;
