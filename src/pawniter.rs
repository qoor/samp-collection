use std::slice::*;
use std::collections::HashMap;

use samp::amx::{Amx, AmxIdent};

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

pub struct PawnAmxIters<'a, T>(HashMap<AmxIdent, PawnIterList<'a, T>>);

impl<'a, T> PawnAmxIters<'a, T> {
	pub fn new() -> Self {
		Self(HashMap::new())
	}

	pub fn add_amx(&mut self, amx: &Amx) {
		self.0.insert(AmxIdent::from(amx.amx().as_ptr()), PawnIterList::new());
	}
	pub fn remove_amx(&mut self, amx: &Amx) {
		self.0.remove(&AmxIdent::from(amx.amx().as_ptr()));
	}
	pub fn get_iter_list(&self, amx: &Amx) -> Option<&PawnIterList<T>> {
		self.0.get(&AmxIdent::from(amx.amx().as_ptr()))
	}
	pub fn get_mut_iter_list(&mut self, amx: &Amx) -> Option<&mut PawnIterList<'a, T>> {
		if let Some(list) = self.0.get_mut(&AmxIdent::from(amx.amx().as_ptr())) {
			Some(list)
		} else {
			None
		}
	}
	
	pub fn get_iter(&self, amx: &Amx, id: i32) -> Option<&Iter<T>> {
		if let Some(list) = self.get_iter_list(amx) {
			list.get_iter(id)
		} else {
			None
		}
	}
	pub fn get_mut_iter(&mut self, amx: &Amx, id: i32) -> Option<&mut Iter<'a, T>> {
		if let Some(list) = self.get_mut_iter_list(amx) {
			list.get_mut_iter(id)
		} else {
			None
		}
	}
}
