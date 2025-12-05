#![no_std]
#![no_main]

use cortex_m::asm::wfi;
use rtic::app;
use stm32f4xx_hal::{
    pac,
    prelude::*,
    serial::{Serial, config::Config},
};

mod device;
mod event;
mod handler;
mod io;
mod protocol;
mod router;
mod system;

pub use event::Event;

#[app(device = stm32f4xx_hal::pac, dispatchers = [EXTI0])]
mod app {
    use stm32f4xx_hal::pac::can1::esr::R;

    use super::*;

    // ----------------------------
    // RTIC 资源（共享数据）
    // ----------------------------
    #[shared]
    struct Shared {
        // router: Router,
    }

    // ----------------------------
    // RTIC 任务局部资源
    // ----------------------------
    #[local]
    struct Local {
        usart1: pac::USART1,
    }

    // ----------------------------
    // 系统初始化（相当于你的 main）
    // ----------------------------
    #[init]
    fn init(ctx: init::Context) -> (Shared, Local) {
        let dp = ctx.device;

        // 配置时钟
        let rcc = dp.RCC.constrain();
        let clocks = rcc.cfgr.sysclk(84.MHz()).freeze();

        // 初始化 UART（示例，可根据你实际情况改）
        let gpioa = dp.GPIOA.split();
        let tx = gpioa.pa9.into_alternate();
        let rx = gpioa.pa10.into_alternate();

        let serial = Serial::new(
            dp.USART1,
            (tx, rx),
            Config::default().baudrate(115_200.bps()),
            clocks,
        );

        // 将 USART1 拿出来，用于中断处理
        let usart1 = serial.free();

        // 启用 USART1 中断
        usart1.cr1.modify(|_, w| w.rxneie().set_bit());
        unsafe { cortex_m::peripheral::NVIC::unmask(pac::Interrupt::USART1) };

        // 初始化你的 Router
        let router = router::Router::new();

        (Shared { router }, Local { usart1 })
    }

    // ----------------------------
    // USART1 中断任务
    // ----------------------------
    #[task(binds = USART1, local = [usart1], shared = [router])]
    fn usart1_irq(ctx: usart1_irq::Context) {
        let usart1 = ctx.local.usart1;

        // 判断是否收到数据
        if usart1.sr.read().rxne().bit_is_set() {
            let byte = usart1.dr.read().dr().bits() as u8;

            // 入队（你原来的 io::data::enqueue_data)
            io::data::enqueue_data(byte);

            // 通知 soft_task 处理逻辑
            process_data::spawn().ok();
        }
    }

    // ----------------------------
    // 软件任务（在中断后执行）
    // ----------------------------
    #[task(shared = [router], priority = 1)]
    async fn process_data(mut ctx: process_data::Context) {
        ctx.shared.router.lock(|router| {
            router.tick();
        });
    }

    // ----------------------------
    // Idle（空闲时执行）
    // ----------------------------
    #[idle]
    fn idle(_: idle::Context) -> ! {
        loop {
            wfi(); // 节能等待中断
        }
    }
}
