#![no_std]
#![no_main]

use core::panic::PanicInfo;

use cortex_m_rt::entry;

mod device;
mod event;
mod handler;
mod protocol;
mod router;
mod system;

pub use crate::event::Event;

#[panic_handler]
fn panic(_: &PanicInfo) -> ! {
    loop {}
}

#[entry]
fn main() -> ! {
    // let mut router = router::Router::new();

    loop {
        // router.tick(); // 执行协议解析 / 状态机
        cortex_m::asm::nop();
    }
}
