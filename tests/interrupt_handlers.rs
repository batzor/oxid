#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(oxid::test_runner)]
#![reexport_test_harness_main = "test_main"]

use core::panic::PanicInfo;
use oxid::{serial_print, serial_println};

#[no_mangle]
pub extern "C" fn _start() -> ! {
    oxid::init();
    test_main();
    loop {}
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    oxid::test_panic_handler(info)
}

#[test_case]
fn test_breakpoint_exception() {
    serial_print!("test_breakpoint_exception...");
    // invoke a breakpoint exception
    x86_64::instructions::interrupts::int3();
    serial_println!("[ok]");
}
