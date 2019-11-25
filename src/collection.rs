use std::collections::HashMap;

use samp::prelude::*;
use samp::amx::AmxIdent;

use crate::idallocator::IdAllocator;
use crate::value::PawnValue;

pub type PawnVec = Vec<PawnValue>;
pub type PawnHashMap = HashMap<PawnValue, PawnValue>;

pub struct ContainerList<T> {
	containers: HashMap<i32, Option<T>>,
	id_allocator: IdAllocator
}

impl<T> ContainerList<T> {
	pub fn new() -> ContainerList<T> {
		Self {
			containers: HashMap::new(),
			id_allocator: IdAllocator::new()
		}
	}

	pub fn add_container(&mut self, container: T) -> i32 {
		let id = self.id_allocator.get_id();
		
		self.containers.insert(id, Some(container));

		id
	}
	pub fn remove_container(&mut self, id: i32) -> Result<(), ()> {
		if !self.containers.remove(&id).is_none() {
			self.id_allocator.remove_id(id);

			Ok(())
		} else {
			Err(())
		}
	}
	pub fn get_container(&self, id: i32) -> Option<&T> {
		if let Some(container) = self.containers.get(&id).unwrap() {
			Some(container)
		} else {
			None
		}
	}
	pub fn get_mut_container(&mut self, id: i32) -> Option<&mut T> {
		if let Some(container) = self.containers.get_mut(&id).unwrap() {
			Some(container)
		} else {
			None
		}
	}
}

pub struct PawnAmxContainers<T>(HashMap<AmxIdent, ContainerList<T>>);

impl<T> PawnAmxContainers<T> {
	pub fn new() -> PawnAmxContainers<T> {
		Self(HashMap::new())
	}

	pub fn add_amx(&mut self, amx: &Amx) {
		let ident = AmxIdent::from(amx.amx().as_ptr());

		self.0.insert(ident, ContainerList::new());
	}
	pub fn remove_amx(&mut self, amx: &Amx) {
		let ident = AmxIdent::from(amx.amx().as_ptr());

		self.0.remove(&ident);
	}
	pub fn get_container_list(&self, amx: &Amx) -> Option<&ContainerList<T>> {
		let ident = AmxIdent::from(amx.amx().as_ptr());

		if let Some(vecs) = self.0.get(&ident) {
			Some(vecs)
		} else {
			None
		}
	}
	pub fn get_mut_container_list(&mut self, amx: &Amx) -> Option<&mut ContainerList<T>> {
		let ident = AmxIdent::from(amx.amx().as_ptr());

		if let Some(vecs) = self.0.get_mut(&ident) {
			Some(vecs)
		} else {
			None
		}
	}
	pub fn get_container(&self, amx: &Amx, id: i32) -> Option<&T> {
		if let Some(vecs) = self.get_container_list(&amx) {
			if let Some(container) = vecs.get_container(id) {
				return Some(container);
			}
		}

		None
	}
	pub fn get_mut_container(&mut self, amx: &Amx, id: i32) -> Option<&mut T> {
		if let Some(vecs) = self.get_mut_container_list(&amx) {
			if let Some(container) = vecs.get_mut_container(id) {
				return Some(container);
			}
		}

		None
	}
}
