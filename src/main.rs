#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(mvos::test_runner)]
#![reexport_test_harness_main = "test_main"]

use core::panic::PanicInfo;
use mvos::println;

#[no_mangle]
pub extern "C" fn _start() -> ! {
    println!("Hello, World {}", "!");

    mvos::init();

    fn stack() {
        stack();
    }

    stack();

    #[cfg(test)]
    test_main();

    println!("I did not crash!");

    loop {}
}

#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    mvos::test_panic_handler(info)
}


#[test_case]
fn trivial_assertion() {
    assert_eq!(1, 1);
}

