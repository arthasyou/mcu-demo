use crate::Event;

mod coin;
mod electromagnet;
mod light;
mod moter;
mod sensor;
mod speaker;

pub trait Device {
    fn tick(&mut self, delta_ms: u32) -> Option<Event>;
}
