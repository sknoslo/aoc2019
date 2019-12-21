use std::collections::VecDeque;
use std::convert::TryFrom;

pub trait IoDevice {
    /**
     * Read a value from an io device
     */
    fn read(&mut self) -> Option<isize>;

    /**
     * Write a value to an io device
     */
    fn write(&mut self, value: isize);
}

#[derive(Debug)]
pub struct QueuedIoDevice {
    queue: VecDeque<isize>,
}

impl QueuedIoDevice {
    pub fn new() -> Self {
        Self {
            queue: VecDeque::new(),
        }
    }
}

impl IoDevice for QueuedIoDevice {
    fn read(&mut self) -> Option<isize> {
        self.queue.pop_back()
    }

    fn write(&mut self, value: isize) {
        self.queue.push_front(value);
    }
}

#[derive(Debug)]
pub struct AsciiDevice {
    non_ascii: VecDeque<isize>,
    queue: VecDeque<char>,
}

impl AsciiDevice {
    pub fn new() -> Self {
        Self {
            non_ascii: VecDeque::new(),
            queue: VecDeque::new(),
        }
    }

    pub fn write_ascii(&mut self, input: &str) {
        input.chars().for_each(|c| self.queue.push_front(c));
    }

    pub fn read_non_ascii(&mut self) -> Option<isize> {
        self.non_ascii.pop_back()
    }

    pub fn get_ascii_image(&mut self) -> String {
        self.queue.iter().rev().collect()
    }
}

impl IoDevice for AsciiDevice {
    fn read(&mut self) -> Option<isize> {
        self.queue.pop_back().map(|c| c as u8 as isize)
    }

    fn write(&mut self, value: isize) {
        match u8::try_from(value) {
            Ok(u) => self.queue.push_front(u as char),
            Err(_) => self.non_ascii.push_front(value),
        };
    }
}
