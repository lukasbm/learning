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
