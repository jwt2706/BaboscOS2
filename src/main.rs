#![no_std] // dont use the standard library, since were doing in raw
#![no_main]

use core::panic::PanicInfo;

// da handler of the panic
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

static BOOT_MSG: &[u8] = b"BabsocOS booted up successfully!";

// this is the entry point, for now
#[no_mangle]
pub extern "C" fn _start() -> ! {
    let vga_buffer = 0xb8000 as *mut u8;
    for (i, &character) in BOOT_MSG.iter().enumerate() {
        unsafe {
            *vga_buffer.offset(i as isize * 2) = character; // character to print
            *vga_buffer.offset(i as isize * 2 + 1) = 0xF; // color
        }
    }
    
    
    loop {}
}
