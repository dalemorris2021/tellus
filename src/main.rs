#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(tellus::test_runner)]
#![reexport_test_harness_main = "test_main"]

use core::panic::PanicInfo;
use tellus::println;

/// this function is the entry point
#[unsafe(no_mangle)]
pub extern "C" fn _start() -> ! {
    println!("Hello, World{}", "!");

    tellus::init();

    x86_64::instructions::interrupts::int3();

    #[cfg(test)]
    test_main();

    println!("It did not crash~");
    loop {}
}

/// this function is called on panic
#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}

/// this function is called on panic
#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    tellus::test_panic_handler(info)
}
