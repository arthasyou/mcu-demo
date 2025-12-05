use core::cell::RefCell;

use cortex_m::interrupt::{self, Mutex};
use heapless::spsc::Queue;

static IO_QUEUE: Mutex<RefCell<Queue<u8, 256>>> = Mutex::new(RefCell::new(Queue::new()));

pub fn enqueue_data(data: u8) {
    interrupt::free(|cs| {
        if let Ok(mut queue) = IO_QUEUE.borrow(cs).try_borrow_mut() {
            let _ = queue.enqueue(data);
        }
    });
}
pub fn dequeue_data() -> Option<u8> {
    let mut result = None;
    interrupt::free(|cs| {
        if let Ok(mut queue) = IO_QUEUE.borrow(cs).try_borrow_mut() {
            result = queue.dequeue();
        }
    });
    result
}
