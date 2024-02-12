use super::log_message::LogMessage::*;
pub use super::log_message::LogMessage;
use colored::Colorize;
use std::collections::VecDeque;
pub struct BufferQueue(VecDeque<LogMessage>);

impl BufferQueue {
    pub fn new() -> BufferQueue {
        BufferQueue(VecDeque::with_capacity(256))
    }
    fn add_message(&mut self, msg: LogMessage) {
        #[cfg(debug_assertions)]
        match &msg {
            Regular(m) => println!("{}", m),
            Good(m) => println!("{}", m.green()),
            Alert(m) => println!("{}", m.yellow()),
            Bad(m) => println!("{}", m.red()),
        }
        self.0.push_back(msg);
    }

    pub fn regular(&mut self, msg: String) {
        self.add_message(Regular(msg));
    }
    pub fn good(&mut self, msg: String) {
        self.add_message(Good(msg));
    }
    pub fn alert(&mut self, msg: String) {
        self.add_message(Alert(msg));
    }
    pub fn bad(&mut self, msg: String) {
        self.add_message(Bad(msg));
    }

    pub fn extract(&mut self)->Vec<LogMessage>{
        self.0.drain(..).rev().collect()
    }
}

use once_cell::sync::Lazy;

pub static mut BUFFER: Lazy<BufferQueue> =
Lazy::new(|| BufferQueue::new());