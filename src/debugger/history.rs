use std::collections::{vec_deque, VecDeque};
use std::fmt;
use std;


#[derive(Debug)]
pub struct LimitedFifoQueue<T> {
    size: usize,
    store: VecDeque<T>
}

impl<T> LimitedFifoQueue<T> {
	pub fn new(size: usize) -> LimitedFifoQueue<T> {
		LimitedFifoQueue {
			size: size,
			store: VecDeque::with_capacity(size),
		}
	}
	pub fn push(&mut self, elem: T) {
        self.store.push_front(elem);
        if self.store.len() > self.size {
            self.store.pop_back();
        }
	}
	pub fn clear(&mut self) {
		self.store.clear();
	}
}


impl<T> IntoIterator for LimitedFifoQueue<T> {
	type Item = T;
	type IntoIter = vec_deque::IntoIter<T>;
	fn into_iter(self) -> Self::IntoIter {
		self.store.into_iter()
	}
}

impl<'a, T> IntoIterator for &'a LimitedFifoQueue<T> {
	type Item = &'a T;
	type IntoIter = vec_deque::Iter<'a, T>;
	fn into_iter(self) -> Self::IntoIter {
		self.store.iter()
	}
}