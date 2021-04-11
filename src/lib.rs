#![no_std]

use panic_halt as _;

extern "C" {
    pub fn vTaskDelay(ticks: u32);
    pub fn consolePutchar(ch: i32) -> i32;
}

#[no_mangle]
pub extern "C" fn appMain() -> i32 {
    for c in "Hello from Rust!\n".as_bytes() {
        unsafe{ consolePutchar(*c as i32); }
    }

    loop {
        unsafe { vTaskDelay(1000); }
    }
}
