pub mod base;
pub mod nb_handler;

use crate::Event;

pub trait Handler {
    /// 向 Handler 发送一个事件
    /// 注意：不能阻塞，不能做长耗时
    fn on_event(&mut self, event: Event);

    /// 每个 tick 调用一次，用于：
    /// - 推进状态机
    /// - 执行 pending 工作
    fn poll(&mut self, delta_ms: u32);
}
