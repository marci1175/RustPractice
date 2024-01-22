use std::{alloc::Layout, collections::HashMap, ffi::CString, ops::Index, pin::Pin, str::Chars};

#[derive(Default, Clone, Copy)]
struct ViewAngles {
    x: f64,
    y: f64,
    z: f64,
}

#[derive(Default, Clone, Copy)]
struct Coordinates {
    x: f64,
    y: f64,
    z: f64,

    viewangles: ViewAngles,
}


impl Index for Coordinates {
    fn index(&self, index: Idx) -> &Self::Output {
        &self[index]
    }

}

unsafe fn pin_main() -> *const u8 {
    
    let ptr = unsafe { std::alloc::alloc(Layout::new::<Pin<Box<i32>>>()) };
    
    ptr.write(*Pin::from(Box::new(10)));

    ptr

}

fn hashmap() {
    let mut hashmap: HashMap<&str, i32> = HashMap::new();

    for num in 0..100 {
        hashmap.insert("Hashmap", num as i32);
    }

}

fn main() {

    unsafe { dbg!(pin_main().read()); }

    hashmap();

    let _ = std::io::stdin().read_line(&mut String::new());
}
