#![allow(dead_code)]
use std::collections::VecDeque;

pub type Merger<T> = Option<Box<Funnel<T>>>;

pub struct Funnel<T> {
    lmerger: Merger<T>,
    rmerger: Merger<T>,
    pub lbuf: VecDeque<T>,
    pub rbuf: VecDeque<T>,
    pub output: VecDeque<T>,
}

impl<T: Ord> Funnel<T> {
    pub fn default() -> Self {
        Funnel {
            lmerger: None,
            rmerger: None,
            lbuf: VecDeque::new(),
            rbuf: VecDeque::new(),
            output: VecDeque::new()
        }
    }

    pub fn new_with_buffers(lbuf: VecDeque<T>, rbuf: VecDeque<T>) -> Self {
        let left_len = lbuf.len();
        let right_len = rbuf.len();

        Funnel {
            lmerger: None,
            rmerger: None,
            lbuf: lbuf,
            rbuf: rbuf,
            output: VecDeque::with_capacity(left_len + right_len)
        }
    }

    pub fn merge_to_completion(&mut self) {
        while !self.lbuf.is_empty() || !self.rbuf.is_empty() {
            self.merge();
        }
    }

    fn merge(&mut self) {
        if self.lbuf.is_empty() && !self.rbuf.is_empty() {
            self.output.append(&mut self.rbuf);
        } else if !self.lbuf.is_empty() && self.rbuf.is_empty() {
            self.output.append(&mut self.lbuf);
        } else {
            let left = self.lbuf.front().unwrap();
            let right = self.rbuf.front().unwrap();

            if left < right {
                self.output.push_back(self.lbuf.pop_front().unwrap()); 
            } else {
                self.output.push_back(self.rbuf.pop_front().unwrap());
            }
        }
    }
}
