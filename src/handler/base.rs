use crate::{Event, handler::Handler};

pub struct BaseHandler;

impl Handler for BaseHandler {
    fn on_event(&mut self, event: Event) {}
    fn poll(&mut self, delta_ms: u32) {}
}
