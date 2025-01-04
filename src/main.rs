#![no_std] // dont use the standard library, since were doing in raw
#![no_main]

mod vga;

use core::panic::PanicInfo;

// da handler of the panic
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}

// this is the entry point, for now
#[no_mangle]
pub extern "C" fn _start() -> ! {
    println!("BaboscOS2 booted successfully!");
    loop {}
}
