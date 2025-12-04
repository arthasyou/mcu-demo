use crate::{device::Device, handler::Handler};

mod tick;

pub struct System<'a> {
    devices: &'a mut [&'a mut dyn Device],
    handler: &'a mut dyn Handler,
}

impl<'a> System<'a> {
    pub fn tick(&mut self, delta_ms: u32) {
        // 1. 驱动所有状态机
        for dev in self.devices.iter_mut() {
            if let Some(event) = dev.tick(delta_ms) {
                // 2. 把事件交给 NB 层
                self.handler.on_event(event);
            }
        }

        // 3. 非阻塞推进 NB 流程
        self.handler.poll(delta_ms);
    }
}
