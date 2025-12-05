use crate::Event;

mod coin;
mod command;
mod electromagnet;
mod led;
mod moter;
mod sensor;
mod speaker;

pub use command::DeviceCommand;

pub trait Device {
    /// 初始化硬件（IO、UART、PWM、ADC 等）
    fn init(&mut self);

    /// 外部命令（未来用 protobuf 控制）
    fn command(&mut self, cmd: DeviceCommand);

    /// 推进内部状态机（典型 MCU 逻辑）
    fn tick(&mut self, delta_ms: u32) -> Option<Event>;

    /// 从内部队列中取出一个事件（可能有多个事件）
    fn next_event(&mut self) -> Option<Event>;
}
