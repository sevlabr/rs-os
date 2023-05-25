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

    use x86_64::registers::control::Cr3;

    let (level_4_page_table, _) = Cr3::read();
    println!(
        "Level 4 page table at: {:?}",
        level_4_page_table.start_address()
    );

    // let ptr = 0x204f88 as *mut u32;

    // unsafe { let x = *ptr; }
    // println!("read worked"); // Ok

    // unsafe { *ptr = 42; }
    // println!("write worked"); // Page fault, read only (because code page)

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
