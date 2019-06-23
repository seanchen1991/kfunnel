#![allow(dead_code)]
use std::collections::VecDeque;

const NULL: usize = !0;

pub struct Funnel<T> {
    // probably going to want to change VecDeques to ArrayVec buffers
    buffers: Vec<VecDeque<T>>,
    boundary: usize,
}

impl<T: Ord + Clone> Funnel<T> {
    pub fn default() -> Self {
        Funnel {
            buffers: vec![VecDeque::new()],
            boundary: 0
        }
    }

    pub fn new_with_buffers(mut input_bufs: Vec<VecDeque<T>>) -> Self {
        // for now assume that input_bufs.len() is a power of 2
        let mut len = input_bufs.len();
        let cap = input_bufs[0].capacity();
        let total_cap = cap * len;

        let mut v = vec![VecDeque::with_capacity(cap); len - 2];
        input_bufs.append(&mut v);

        input_bufs.push(VecDeque::with_capacity(total_cap));
        input_bufs.reverse();
        len = input_bufs.len() - 1;

        Funnel {
            buffers: input_bufs,
            boundary: len
        }
    }

    pub fn fill_to_completion(&mut self) {
        self.fill(0);
    }

    fn fill(&mut self, index: usize) {
        assert!(index <= self.boundary);
         
        while self.buffers[index].len() < self.buffers[index].capacity() {
            let left = if 2 * index + 1 <= self.boundary { 2 * index + 1 } else { NULL };
            let right = if 2 * index + 2 <= self.boundary { 2 * index + 2 } else { NULL };

            if left != NULL {
                if self.buffers[left].is_empty() {
                    self.fill(left);
                }
            }

            if right != NULL {
                if self.buffers[right].is_empty() {
                    self.fill(right);
                }
            }
            
            if left != NULL && right != NULL {
                self.merge(index, left, right); 
            } else if left != NULL {
                self.buffers[index].append(&mut self.buffers[left]);
            } else {
                self.buffers[index].append(&mut self.buffers[right]);
            }
        }
    }

    fn merge(&mut self, parent: usize, left: usize, right: usize) {
        // check if left buf is empty and right buf is not empty
        if self.buffers[left].is_empty() && !self.buffers[right].is_empty() {
            self.buffers[parent].append(&mut self.buffers[right]);
        }
        // check if left buf is not empty and right bug is empty
        else if !self.buffers[left].is_empty() && self.buffers[right].is_empty() {
            self.buffers[parent].append(&mut self.buffers[left]);
        } 
        // handle case when neither buf is empty
        else {
            let l = self.buffers[left].front().unwrap();
            let r = self.buffers[right].front().unwrap();

            if l < r {
                self.buffers[parent].push_back(self.buffers[left].pop_front().unwrap());
            } else {
                self.buffers[parent].push_back(self.buffers[right].pop_front().unwrap());
            }
        }
    }
}
