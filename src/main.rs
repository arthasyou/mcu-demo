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
    let mut rcc = rcc.freeze(Config::default(), &mut flash.acr);

    let mut afio = dp.AFIO.constrain(&mut rcc);

    // 分解 GPIO
    let mut gpioa = dp.GPIOA.split(&mut rcc);
    let mut gpiob = dp.GPIOB.split(&mut rcc);
    let mut gpioc = dp.GPIOC.split(&mut rcc);
    let mut gpiod = dp.GPIOD.split(&mut rcc);
    let mut gpioe = dp.GPIOE.split(&mut rcc);

    // 禁用 JTAG，释放 PA15 / PB3 / PB4
    let (pa15_dbg, pb3_dbg, pb4_dbg) = afio.mapr.disable_jtag(gpioa.pa15, gpiob.pb3, gpiob.pb4);

    // ===== GPIOA =====
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
    let mut pa15 = pa15_dbg.into_push_pull_output(&mut gpioa.crh);

    // ===== GPIOB =====
    let mut pb0 = gpiob.pb0.into_push_pull_output(&mut gpiob.crl);
    let mut pb1 = gpiob.pb1.into_push_pull_output(&mut gpiob.crl);
    let mut pb2 = gpiob.pb2.into_push_pull_output(&mut gpiob.crl);
    let mut pb3 = pb3_dbg.into_push_pull_output(&mut gpiob.crl);
    let mut pb4 = pb4_dbg.into_push_pull_output(&mut gpiob.crl);
    let mut pb5 = gpiob.pb5.into_push_pull_output(&mut gpiob.crl);
    let mut pb6 = gpiob.pb6.into_push_pull_output(&mut gpiob.crl);
    let mut pb7 = gpiob.pb7.into_push_pull_output(&mut gpiob.crl);
    let mut pb8 = gpiob.pb8.into_push_pull_output(&mut gpiob.crh);
    let mut pb9 = gpiob.pb9.into_push_pull_output(&mut gpiob.crh);
    let mut pb10 = gpiob.pb10.into_push_pull_output(&mut gpiob.crh);
    let mut pb11 = gpiob.pb11.into_push_pull_output(&mut gpiob.crh);
    let mut pb12 = gpiob.pb12.into_push_pull_output(&mut gpiob.crh);
    let mut pb13 = gpiob.pb13.into_push_pull_output(&mut gpiob.crh);
    let mut pb14 = gpiob.pb14.into_push_pull_output(&mut gpiob.crh);
    let mut pb15 = gpiob.pb15.into_push_pull_output(&mut gpiob.crh);

    // ===== GPIOC =====
    let mut pc0 = gpioc.pc0.into_push_pull_output(&mut gpioc.crl);
    let mut pc1 = gpioc.pc1.into_push_pull_output(&mut gpioc.crl);
    let mut pc2 = gpioc.pc2.into_push_pull_output(&mut gpioc.crl);
    let mut pc3 = gpioc.pc3.into_push_pull_output(&mut gpioc.crl);
    let mut pc4 = gpioc.pc4.into_push_pull_output(&mut gpioc.crl);
    let mut pc5 = gpioc.pc5.into_push_pull_output(&mut gpioc.crl);
    let mut pc6 = gpioc.pc6.into_push_pull_output(&mut gpioc.crl);
    let mut pc7 = gpioc.pc7.into_push_pull_output(&mut gpioc.crl);
    let mut pc8 = gpioc.pc8.into_push_pull_output(&mut gpioc.crh);
    let mut pc9 = gpioc.pc9.into_push_pull_output(&mut gpioc.crh);
    let mut pc10 = gpioc.pc10.into_push_pull_output(&mut gpioc.crh);
    let mut pc11 = gpioc.pc11.into_push_pull_output(&mut gpioc.crh);
    let mut pc12 = gpioc.pc12.into_push_pull_output(&mut gpioc.crh);
    let mut pc13 = gpioc.pc13.into_push_pull_output(&mut gpioc.crh);
    let mut pc14 = gpioc.pc14.into_push_pull_output(&mut gpioc.crh);
    let mut pc15 = gpioc.pc15.into_push_pull_output(&mut gpioc.crh);

    // ===== GPIOD =====
    let mut pd0 = gpiod.pd0.into_push_pull_output(&mut gpiod.crl);
    let mut pd1 = gpiod.pd1.into_push_pull_output(&mut gpiod.crl);
    let mut pd2 = gpiod.pd2.into_push_pull_output(&mut gpiod.crl);
    let mut pd3 = gpiod.pd3.into_push_pull_output(&mut gpiod.crl);
    let mut pd4 = gpiod.pd4.into_push_pull_output(&mut gpiod.crl);
    let mut pd5 = gpiod.pd5.into_push_pull_output(&mut gpiod.crl);
    let mut pd6 = gpiod.pd6.into_push_pull_output(&mut gpiod.crl);
    let mut pd7 = gpiod.pd7.into_push_pull_output(&mut gpiod.crl);
    let mut pd8 = gpiod.pd8.into_push_pull_output(&mut gpiod.crh);
    let mut pd9 = gpiod.pd9.into_push_pull_output(&mut gpiod.crh);
    let mut pd10 = gpiod.pd10.into_push_pull_output(&mut gpiod.crh);
    let mut pd11 = gpiod.pd11.into_push_pull_output(&mut gpiod.crh);
    let mut pd12 = gpiod.pd12.into_push_pull_output(&mut gpiod.crh);
    let mut pd13 = gpiod.pd13.into_push_pull_output(&mut gpiod.crh);
    let mut pd14 = gpiod.pd14.into_push_pull_output(&mut gpiod.crh);
    let mut pd15 = gpiod.pd15.into_push_pull_output(&mut gpiod.crh);

    // ===== GPIOE =====
    let mut pe0 = gpioe.pe0.into_push_pull_output(&mut gpioe.crl);
    let mut pe1 = gpioe.pe1.into_push_pull_output(&mut gpioe.crl);
    let mut pe2 = gpioe.pe2.into_push_pull_output(&mut gpioe.crl);
    let mut pe3 = gpioe.pe3.into_push_pull_output(&mut gpioe.crl);
    let mut pe4 = gpioe.pe4.into_push_pull_output(&mut gpioe.crl);
    let mut pe5 = gpioe.pe5.into_push_pull_output(&mut gpioe.crl);
    let mut pe6 = gpioe.pe6.into_push_pull_output(&mut gpioe.crl);
    let mut pe7 = gpioe.pe7.into_push_pull_output(&mut gpioe.crl);
    let mut pe8 = gpioe.pe8.into_push_pull_output(&mut gpioe.crh);
    let mut pe9 = gpioe.pe9.into_push_pull_output(&mut gpioe.crh);
    let mut pe10 = gpioe.pe10.into_push_pull_output(&mut gpioe.crh);
    let mut pe11 = gpioe.pe11.into_push_pull_output(&mut gpioe.crh);
    let mut pe12 = gpioe.pe12.into_push_pull_output(&mut gpioe.crh);
    let mut pe13 = gpioe.pe13.into_push_pull_output(&mut gpioe.crh);
    let mut pe14 = gpioe.pe14.into_push_pull_output(&mut gpioe.crh);
    let mut pe15 = gpioe.pe15.into_push_pull_output(&mut gpioe.crh);

    loop {
        // ===== A =====
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

        // ===== B =====
        pb0.set_high();
        delay(5_000_0);
        pb0.set_low();
        pb1.set_high();
        delay(5_000_0);
        pb1.set_low();
        pb2.set_high();
        delay(5_000_0);
        pb2.set_low();
        pb3.set_high();
        delay(5_000_0);
        pb3.set_low();
        pb4.set_high();
        delay(5_000_0);
        pb4.set_low();
        pb5.set_high();
        delay(5_000_0);
        pb5.set_low();
        pb6.set_high();
        delay(5_000_0);
        pb6.set_low();
        pb7.set_high();
        delay(5_000_0);
        pb7.set_low();
        pb8.set_high();
        delay(5_000_0);
        pb8.set_low();
        pb9.set_high();
        delay(5_000_0);
        pb9.set_low();
        pb10.set_high();
        delay(5_000_0);
        pb10.set_low();
        pb11.set_high();
        delay(5_000_0);
        pb11.set_low();
        pb12.set_high();
        delay(5_000_0);
        pb12.set_low();
        pb13.set_high();
        delay(5_000_0);
        pb13.set_low();
        pb14.set_high();
        delay(5_000_0);
        pb14.set_low();
        pb15.set_high();
        delay(5_000_0);
        pb15.set_low();

        // ===== C =====
        pc0.set_high();
        delay(5_000_0);
        pc0.set_low();
        pc1.set_high();
        delay(5_000_0);
        pc1.set_low();
        pc2.set_high();
        delay(5_000_0);
        pc2.set_low();
        pc3.set_high();
        delay(5_000_0);
        pc3.set_low();
        pc4.set_high();
        delay(5_000_0);
        pc4.set_low();
        pc5.set_high();
        delay(5_000_0);
        pc5.set_low();
        pc6.set_high();
        delay(5_000_0);
        pc6.set_low();
        pc7.set_high();
        delay(5_000_0);
        pc7.set_low();
        pc8.set_high();
        delay(5_000_0);
        pc8.set_low();
        pc9.set_high();
        delay(5_000_0);
        pc9.set_low();
        pc10.set_high();
        delay(5_000_0);
        pc10.set_low();
        pc11.set_high();
        delay(5_000_0);
        pc11.set_low();
        pc12.set_high();
        delay(5_000_0);
        pc12.set_low();
        pc13.set_high();
        delay(5_000_0);
        pc13.set_low();
        pc14.set_high();
        delay(5_000_0);
        pc14.set_low();
        pc15.set_high();
        delay(5_000_0);
        pc15.set_low();

        // ===== D =====
        pd0.set_high();
        delay(5_000_0);
        pd0.set_low();
        pd1.set_high();
        delay(5_000_0);
        pd1.set_low();
        pd2.set_high();
        delay(5_000_0);
        pd2.set_low();
        pd3.set_high();
        delay(5_000_0);
        pd3.set_low();
        pd4.set_high();
        delay(5_000_0);
        pd4.set_low();
        pd5.set_high();
        delay(5_000_0);
        pd5.set_low();
        pd6.set_high();
        delay(5_000_0);
        pd6.set_low();
        pd7.set_high();
        delay(5_000_0);
        pd7.set_low();
        pd8.set_high();
        delay(5_000_0);
        pd8.set_low();
        pd9.set_high();
        delay(5_000_0);
        pd9.set_low();
        pd10.set_high();
        delay(5_000_0);
        pd10.set_low();
        pd11.set_high();
        delay(5_000_0);
        pd11.set_low();
        pd12.set_high();
        delay(5_000_0);
        pd12.set_low();
        pd13.set_high();
        delay(5_000_0);
        pd13.set_low();
        pd14.set_high();
        delay(5_000_0);
        pd14.set_low();
        pd15.set_high();
        delay(5_000_0);
        pd15.set_low();

        // ===== E =====
        pe0.set_high();
        delay(5_000_0);
        pe0.set_low();
        pe1.set_high();
        delay(5_000_0);
        pe1.set_low();
        pe2.set_high();
        delay(5_000_0);
        pe2.set_low();
        pe3.set_high();
        delay(5_000_0);
        pe3.set_low();
        pe4.set_high();
        delay(5_000_0);
        pe4.set_low();
        pe5.set_high();
        delay(5_000_0);
        pe5.set_low();
        pe6.set_high();
        delay(5_000_0);
        pe6.set_low();
        pe7.set_high();
        delay(5_000_0);
        pe7.set_low();
        pe8.set_high();
        delay(5_000_0);
        pe8.set_low();
        pe9.set_high();
        delay(5_000_0);
        pe9.set_low();
        pe10.set_high();
        delay(5_000_0);
        pe10.set_low();
        pe11.set_high();
        delay(5_000_0);
        pe11.set_low();
        pe12.set_high();
        delay(5_000_0);
        pe12.set_low();
        pe13.set_high();
        delay(5_000_0);
        pe13.set_low();
        pe14.set_high();
        delay(5_000_0);
        pe14.set_low();
        pe15.set_high();
        delay(5_000_0);
        pe15.set_low();
    }
}
