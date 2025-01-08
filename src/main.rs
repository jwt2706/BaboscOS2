#![no_std] // dont use the standard library, since were doing in raw
#![no_main]

use core::panic::PanicInfo;

mod vga;

// this is the entry point, for now
#[no_mangle]
pub extern "C" fn _start() -> ! {
    print!("BaboscOS2 booted successfully!\nv0.1.0");
    loop {}
}

// da handler of the panic
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}

/*
#![feature(custom_test_framework)]
#![test_runner(crate::test_runner)]

#[cfg(test)]
pub fn test_runner(tests: &[&dyn Fn()]) {
    prinln!("Running {} tests", tests.len());
    for test in tests {
        test();
    }
}
*/
