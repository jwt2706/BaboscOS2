#![no_std] // dont use the standard library, since were doing in raw
#![no_main]

mod vga;

use core::panic::PanicInfo;

// da handler of the panic
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

// this is the entry point, for now
#[no_mangle]
pub extern "C" fn _start() -> ! {
    use core::fmt::Write;
    //write!(vga::WRITER.lock(), "The answer to the universe {}.", 42).unwrap();
    vga::WRITER.lock().write_str("\nBaboscOS2 booted successfully!").unwrap();
    loop {}
}
