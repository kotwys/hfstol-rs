use alloc::vec::Vec;
use super::{Symbol, Weight, NO_SYMBOL_NUMBER};

const SIZE: usize = 1_000;

pub struct Cursor {
    buffer: [Symbol; SIZE],
    position: usize,
    weight: Weight,
}

impl Cursor {
    pub fn new() -> Cursor {
        Cursor {
            buffer: [NO_SYMBOL_NUMBER; SIZE],
            position: 0,
            weight: 0.0,
        }
    }

    pub fn push(&mut self, sym: Symbol) {
        if self.overflowed() {
            return
        }
        // Safe because self.position is always kept within the range.
        unsafe {
            *self.buffer.get_unchecked_mut(self.position) = sym;
        }
        self.position += 1;
    }

    pub fn add_weight(&mut self, weight: Weight) {
        self.weight += weight;
    }

    pub fn take_weight(&mut self, weight: Weight) {
        self.weight -= weight;
    }

    pub fn overflowed(&self) -> bool {
        self.position >= SIZE
    }

    pub fn retract(&mut self, n: usize) {
        self.position = match self.position.checked_sub(n) {
            Some(p) => p,
            _ => 0
        };
    }

    pub fn dump(&self) -> (Vec<Symbol>, Weight) {
        // Safe because self.position is always kept within the range.
        let vec = unsafe {
            self.buffer.get_unchecked(..self.position)
        }.to_vec();
        (vec, self.weight)
    }
}
