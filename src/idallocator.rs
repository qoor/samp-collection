use std::collections::BinaryHeap;
use std::cmp::Reverse;

pub struct IdAllocator {
	remove_ids: BinaryHeap<Reverse<i32>>,
	id_max: i32
}

impl IdAllocator {
	pub fn new() -> IdAllocator {
		Self {
			remove_ids: BinaryHeap::new(),
			id_max: -1
		}
	}

	pub fn get_id(&mut self) -> i32 {
		if !self.remove_ids.is_empty() {
			let Reverse(value) = self.remove_ids.pop().unwrap();

			value
		} else {
			self.id_max += 1;

			self.id_max
		}
	}
	pub fn remove_id(&mut self, id: i32) {
		self.remove_ids.push(Reverse(id));
	}
	pub fn clear(&mut self) {
		self.remove_ids.clear();
		self.id_max = 0;
	}
}