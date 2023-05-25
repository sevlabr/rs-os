#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(rs_os::test_runner)]
#![reexport_test_harness_main = "test_main"]

use core::panic::PanicInfo;
use rs_os::println;

#[allow(clippy::empty_loop)]
#[no_mangle]
pub extern "C" fn _start() -> ! {
    println!("Hello World{}", "!");

    rs_os::init();

    #[cfg(test)]
    test_main();

    println!("It did not crash!");
    rs_os::hlt_loop();
}

/// This function is called on panic.
#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    rs_os::hlt_loop();
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    rs_os::test_panic_handler(info)
}

#[test_case]
fn trivial_assertion() {
    assert_eq!(1, 1);
}
