#![no_std]
#![no_main]

use core::panic::PanicInfo;

use cortex_m::asm::{nop};
use cortex_m_rt::entry;
use stm32f4xx_hal as hal;
use hal::pac;

#[entry]
fn main() -> ! {
    loop {
        nop();
    }
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
