#![feature(panic_implementation)]
#![feature(alloc_error_handler)]
#![no_main]
#![no_std]

#[macro_use]
extern crate cortex_m_rt as rt;

use core::panic::PanicInfo;

entry!(main);

fn main() -> ! {
    panic!()
}

#[panic_implementation]
fn panic(info: &PanicInfo) -> ! {
    loop {}
}
