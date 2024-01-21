use std::{alloc::Layout, pin::Pin, ffi::CString};
use cxx::CxxString;
use crate::cpp_link::{hello, times_two};

#[cxx::bridge]
pub mod cpp_link {
    unsafe extern "C++" {
        pub fn hello() -> ();
        pub fn times_two(number : i32) -> i32;
        
        pub fn reverse_string(string : &CxxString) -> &CxxString;
    }
}

#[no_mangle]
unsafe fn pin_main() -> *const u8 {
    
    let ptr = unsafe { std::alloc::alloc(Layout::new::<Pin<Box<i32>>>()) };
    
    ptr.write(*Pin::from(Box::new(10)));

    ptr

}

fn main() {
    unsafe {

        //Hello from CPP
        hello();

        //Times_two from cpp
        dbg!(times_two(100));

        let sample_string: String = "Varga".to_string();

        //dbg!(reverse_string(CString::from_vec_unchecked(sample_string.into_bytes())));

        // dbg!(pin_main().read());

        let _ = std::io::stdin().read_line(&mut String::new());
    }

}
