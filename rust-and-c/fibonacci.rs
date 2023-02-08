#![feature(lang_items, core_intrinsics)]
#![no_std]
#![no_main]

use core::intrinsics;
use core::panic::PanicInfo;

#[panic_handler]
fn panic(_panic: &PanicInfo<'_>) -> ! {
    intrinsics::abort();
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
