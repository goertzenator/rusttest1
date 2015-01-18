

#![crate_type = "dylib"]
//#![crate_type = "staticlib"]

#[no_mangle]
pub extern fn hello() {
	println!("hello from rust!");
}



//#[no_mangle]
//  pub extern fn rustlib_increment(x: i32) -> i32 { x + 1 }
