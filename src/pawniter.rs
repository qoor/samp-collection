use std::slice::*;
use std::collections::HashMap;

use crate::value::*;
use crate::idallocator::IdAllocator;

pub struct PawnIterList<'a, T> {
	iterators: HashMap<i32, Iter<'a, T>>,
	id_allocator: IdAllocator
}

impl<'a, T> PawnIterList<'a, T> {
	pub fn new() -> Self {
		Self {
			iterators: HashMap::new(),
			id_allocator: IdAllocator::new()
		}
	}

	pub fn add_iter(&mut self, iterator: Iter<'a, T>) -> i32 {
		let id = self.id_allocator.get_id();

		self.iterators.insert(id, iterator);

		id
	}
	pub fn remove_iter(&mut self, id: i32) -> Result<(), ()> {
		if self.iterators.remove(&id).is_some() {
			self.id_allocator.remove_id(id);
			
			Ok(())
		} else {
			Err(())
		}
	}
	pub fn get_iter(&self, id: i32) -> Option<&Iter<'a, T>> {
		self.iterators.get(&id)
	}
	pub fn get_mut_iter(&mut self, id: i32) -> Option<&mut Iter<'a, T>> {
		self.iterators.get_mut(&id)
	}
	pub fn set_iter(&mut self, id: i32, iterator: Iter<'a, T>) {
		self.iterators.insert(id, iterator).is_some();
	}
}
