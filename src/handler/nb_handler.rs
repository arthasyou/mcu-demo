// use heapless::spsc::{Consumer, Producer, Queue};
// use nb::{Error as NbError, Result as NbResult};

// use crate::{Event, handler::Handler};

// pub struct NbHandler<T: Handler, const N: usize> {
//     inner: T,
//     prod: Producer<'static, Event, N>,
//     cons: Consumer<'static, Event, N>,
// }

// impl<T: Handler, const N: usize> NbHandler<T, N> {
//     /// 构造函数（初始化队列）
//     pub fn new(inner: T, queue: &'static mut Queue<Event, N>) -> Self {
//         let (prod, cons) = queue.split();
//         Self { inner, prod, cons }
//     }

//     /// 推入事件（非阻塞）
//     pub fn push_event(&mut self, event: Event) -> NbResult<(), ()> {
//         if self.prod.enqueue(event).is_err() {
//             Err(NbError::WouldBlock)
//         } else {
//             Ok(())
//         }
//     }

//     /// 尝试处理一个事件
//     pub fn try_process_one(&mut self) -> NbResult<(), ()> {
//         if let Some(ev) = self.cons.dequeue() {
//             self.inner.on_event(ev);
//             Ok(())
//         } else {
//             Err(NbError::WouldBlock)
//         }
//     }

//     /// tick 驱动
//     pub fn poll(&mut self, delta_ms: u32) {
//         self.inner.poll(delta_ms);
//     }
// }
