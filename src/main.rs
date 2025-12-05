#![no_std]
#![no_main]

use core::panic::PanicInfo;

use cortex_m_rt::entry;
use stm32f4xx_hal::interrupt;

mod device;
mod event;
mod handler;
mod io;
mod protocol;
mod router;
mod system;
pub use crate::event::Event;

#[panic_handler]
fn panic(_: &PanicInfo) -> ! {
    loop {
        cortex_m::asm::wfi(); // 让cpu进入低功耗等待状态
    }
}

#[entry]
fn main() -> ! {
    // let mut router = router::Router::new();

    loop {
        // router.tick(); // 执行协议解析 / 状态机
        cortex_m::asm::nop();
    }
}

#[interrupt]
fn USART1() {
    let usart = unsafe { &*stm32f4xx_hal::pac::USART1::ptr() };

    // 读取 SR 的 RXNE 标志
    if usart.sr().read().rxne().bit_is_set() {
        // 读取 DR 来清除 RXNE 标志
        let byte = usart.dr().read().dr().bits() as u8;
        io::data::enqueue_data(byte);
    }
}
