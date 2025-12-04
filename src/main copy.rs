#![no_std]
#![no_main]

use core::panic::PanicInfo;

use cortex_m::asm::delay;
use cortex_m_rt::entry;
use stm32f1xx_hal::{pac, prelude::*, rcc::Config};

#[panic_handler]
fn panic(_: &PanicInfo) -> ! {
    loop {}
}

#[entry]
fn main() -> ! {
    let dp = pac::Peripherals::take().unwrap();

    let mut flash = dp.FLASH.constrain();
    let rcc = dp.RCC.constrain();

    // ⭕ freeze 会返回新的 rcc，必须接住，否则 rcc 会被 move
    let mut rcc = rcc.freeze(Config::default(), &mut flash.acr);

    let mut afio = dp.AFIO.constrain(&mut rcc);

    // 分解 GPIO
    let mut gpioa = dp.GPIOA.split(&mut rcc);
    let gpiob = dp.GPIOB.split(&mut rcc);

    // 禁用 JTAG，释放 PA15 PB3 PB4
    let (pa15_dbg, _pb3_dbg, _pb4_dbg) = afio.mapr.disable_jtag(gpioa.pa15, gpiob.pb3, gpiob.pb4);

    // A 口所有输出
    let mut pa0 = gpioa.pa0.into_push_pull_output(&mut gpioa.crl);
    let mut pa1 = gpioa.pa1.into_push_pull_output(&mut gpioa.crl);
    let mut pa2 = gpioa.pa2.into_push_pull_output(&mut gpioa.crl);
    let mut pa3 = gpioa.pa3.into_push_pull_output(&mut gpioa.crl);
    let mut pa4 = gpioa.pa4.into_push_pull_output(&mut gpioa.crl);
    let mut pa5 = gpioa.pa5.into_push_pull_output(&mut gpioa.crl);
    let mut pa6 = gpioa.pa6.into_push_pull_output(&mut gpioa.crl);
    let mut pa7 = gpioa.pa7.into_push_pull_output(&mut gpioa.crl);

    let mut pa8 = gpioa.pa8.into_push_pull_output(&mut gpioa.crh);
    let mut pa9 = gpioa.pa9.into_push_pull_output(&mut gpioa.crh);
    let mut pa10 = gpioa.pa10.into_push_pull_output(&mut gpioa.crh);
    let mut pa11 = gpioa.pa11.into_push_pull_output(&mut gpioa.crh);
    let mut pa12 = gpioa.pa12.into_push_pull_output(&mut gpioa.crh);

    // PA15
    let mut pa15 = pa15_dbg.into_push_pull_output(&mut gpioa.crh);

    loop {
        pa0.set_high();
        delay(5_000_0);
        pa0.set_low();
        pa1.set_high();
        delay(5_000_0);
        pa1.set_low();
        pa2.set_high();
        delay(5_000_0);
        pa2.set_low();
        pa3.set_high();
        delay(5_000_0);
        pa3.set_low();
        pa4.set_high();
        delay(5_000_0);
        pa4.set_low();
        pa5.set_high();
        delay(5_000_0);
        pa5.set_low();
        pa6.set_high();
        delay(5_000_0);
        pa6.set_low();
        pa7.set_high();
        delay(5_000_0);
        pa7.set_low();

        pa8.set_high();
        delay(5_000_0);
        pa8.set_low();
        pa9.set_high();
        delay(5_000_0);
        pa9.set_low();
        pa10.set_high();
        delay(5_000_0);
        pa10.set_low();
        pa11.set_high();
        delay(5_000_0);
        pa11.set_low();
        pa12.set_high();
        delay(5_000_0);
        pa12.set_low();

        pa15.set_high();
        delay(5_000_0);
        pa15.set_low();
    }
}
