// #![feature(lang_items)]
#![no_std]
#![no_main]
// use core::intrinsics;
use core::panic::PanicInfo;

#[panic_handler]
fn panic(_panic: &PanicInfo<'_>) -> ! {
    loop {}
}

#[no_mangle]
pub extern "C" fn fibonacci(n: i32) -> u64 {
    if n <= 0 {
        panic!("no negative arguments");
    }
    match n {
        1 | 2 => 1,
        _ => fibonacci(n - 1) + fibonacci(n - 2),
    }
}


#[lang = "eh_personality"]
extern "C" fn eh_personality() {}
// #[lang = "eh_personality"] extern fn rust_eh_personality() {}
// #[lang = "panic_impl"] extern fn rust_begin_panic(info: &PanicInfo) -> ! { unsafe { intrinsics::abort() } }
// #[no_mangle] pub extern fn rust_eh_register_frames () {}
// #[no_mangle] pub extern fn rust_eh_unregister_frames () {}
