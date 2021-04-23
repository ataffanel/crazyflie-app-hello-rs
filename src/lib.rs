#![no_std]
#![feature(alloc_error_handler)]
#![allow(non_camel_case_types)]

extern crate freertos_rs;

use freertos_rs::*;
use panic_halt as _;

#[repr(u8)]
enum c_void {
    __variant1,
    __variant2,
}

extern "C" {
    fn consolePutchar(ch: i32) -> i32;
    fn pvPortMalloc(size: u32) -> *mut c_void;
    fn vPortFree(p: *mut c_void);
}

macro_rules! console_print {
    ($msg:expr) => {
        for c in $msg.as_bytes() {
            unsafe{ consolePutchar(*c as i32); }
        }
    }
}

#[alloc_error_handler]
fn foo(_: core::alloc::Layout) -> ! {
    panic!("OOM!");
}

use core::alloc::{GlobalAlloc, Layout};

pub struct FreeRtosAllocator;

unsafe impl GlobalAlloc for FreeRtosAllocator {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        pvPortMalloc(layout.size() as u32) as *mut u8
    }

    unsafe fn dealloc(&self, ptr: *mut u8, _layout: Layout) {
        vPortFree(ptr as *mut c_void)
    }
}

#[global_allocator]
static GLOBAL: FreeRtosAllocator = FreeRtosAllocator;

mod app;
