// TODO: macro create_extern_c_wrapper!(in_func, out_func) {
//     std::panic::catch_unwind(|| {
//         in_func();
//     });
//     if result.is_err() {
//         eprintln!("error: rust panic");
//     }
// }

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
