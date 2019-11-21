use std::collections::HashMap;

use samp::prelude::*;
use samp::amx::AmxIdent;

use crate::value::PawnValue;
use crate::idallocator::IdAllocator;

pub enum Container {
	PawnVec(Vec<PawnValue>),
	PawnHashMap(HashMap<PawnValue, PawnValue>)
}

pub struct ContainerList {
	containers: HashMap<i32, Option<Container>>,
	id_allocator: IdAllocator
}

impl ContainerList {
	pub fn new() -> ContainerList {
		Self {
			containers: HashMap::new(),
			id_allocator: IdAllocator::new()
		}
	}

	pub fn add_container(&mut self, collection: Container) -> i32 {
		let id = self.id_allocator.get_id();
		
		self.containers.insert(id, Some(collection));

		id
	}
	pub fn remove_container(&mut self, id: i32) -> Result<(), ()> {
		if !self.containers.remove(&id).is_none() {
			Ok(())
		} else {
			Err(())
		}
	}
	pub fn get_mut_container(&mut self, id: i32) -> Result<&mut Container, ()> {
		if let Some(container) = self.containers.get_mut(&id).unwrap() {
			Ok(container)
		} else {
			Err(())
		}
	}
}

pub struct PawnAmxContainers(HashMap<AmxIdent, ContainerList>);

impl PawnAmxContainers {
	pub fn new() -> PawnAmxContainers {
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
	pub fn get_mut_vecs(&mut self, amx: &Amx) -> Result<&mut ContainerList, ()> {
		let ident = AmxIdent::from(amx.amx().as_ptr());

		if let Some(vecs) = self.0.get_mut(&ident) {
			Ok(vecs)
		} else {
			Err(())
		}
	}
	pub fn get_mut_container(&mut self, amx: &Amx, id: i32) -> Result<&mut Container, ()> {
		if let Some(vecs) = self.get_mut_vecs(&amx).ok() {
			if let Some(container) = vecs.get_mut_container(id).ok() {
				return Ok(container);
			}
		}

		Err(())
	}
}
