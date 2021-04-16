extern crate alloc;

use crate::*;

#[no_mangle]
pub extern "C" fn appMain() -> i32 {
    console_print!("Hello From Rust!\n");

    Task::new().name("app").start(|| {
        loop {
            CurrentTask::delay(Duration::ms(1000));
            console_print!("Rust Still Alive!\n");
        }
    }).unwrap();

    loop { CurrentTask::delay(Duration::ms(1)); }
}
