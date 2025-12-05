use crate::{
    Event,
    device::{
        Device,
        command::{DeviceCommand, LedCommand},
    },
};

pub struct LedDevice<const N: usize> {
    /// 当前每个灯的亮灭状态
    states: [bool; N],
}

impl<const N: usize> LedDevice<N> {
    pub fn new() -> Self {
        Self { states: [false; N] }
    }
}

impl<const N: usize> LedDevice<N> {
    pub fn handle_command(&mut self, cmd: LedCommand) {
        match cmd {
            LedCommand::Set(ids) => self.apply_set_all(ids),
        }
    }

    fn apply_set_all(&mut self, ids: &[u8]) {
        // 1. 全部灭
        self.states = [false; N];

        // 2. 点亮指定灯
        for &id in ids {
            if (id as usize) < N {
                self.states[id as usize] = true;
            }
        }
    }
}

impl<const N: usize> Device for LedDevice<N> {
    fn init(&mut self) {
        // 上电默认全部熄灭
        self.states = [false; N];
    }

    fn command(&mut self, cmd: DeviceCommand) {
        match cmd {
            DeviceCommand::Led(led_cmd) => {
                self.handle_command(led_cmd);
            }
            // 不是给LED的命令，忽略
            _ => {}
        }
    }

    fn next_event(&mut self) -> Option<Event> {
        None
    }

    fn tick(&mut self, _delta_ms: u32) -> Option<Event> {
        None
    }
}
