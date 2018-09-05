extern {
    fn call_me_back(msg: *const u8);
}

#[no_mangle]
pub extern fn hello_rust() -> *const u8 {
    unsafe { call_me_back("Hey\0".as_ptr()); }
    "Hello, world! 🐸\0".as_ptr()
}
