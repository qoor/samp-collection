use std::collections::HashMap;

use samp::amx::AmxIdent;
use log::error;

use crate::value::PawnValue;
use crate::idallocator::IdAllocator;

/*pub struct PawnVec(Vec<PawnValue>);

impl PawnVec {
	pub fn new() -> PawnVec {
		Self(Vec::new())
	}
	pub fn with_capacity(capacity: i32) -> PawnVec {
		Self(Vec::with_capacity(capacity as usize))
	}
	pub unsafe fn from_raw_parts(ptr: *mut PawnValue, length: i32, capacity: i32) -> PawnVec {
		Self(Vec::from_raw_parts(ptr, length as usize, capacity as usize))
	}

	pub fn capacity(&self) -> i32 {
		self.0.capacity() as i32
	}
	pub fn reserve(&mut self, additional: i32) {
		self.0.reserve(additional as usize);
	}
	pub fn reserve_exact(&mut self, additional: i32) {
		self.0.reserve_exact(additional as usize);
	}
	pub fn into_boxed_slice(mut self) -> Box<[PawnValue]> {
		self.0.into_boxed_slice()
	}
	pub fn truncate(&mut self, len: i32) {
		self.0.truncate(len as usize);
	}
	pub fn as_slice(&self) -> &[PawnValue] {
		self.0.as_slice()
	}
	pub fn as_mut_slice(&mut self) -> &mut [PawnValue] {
		self.0.as_mut_slice()
	}
}*/

pub struct PawnVecs {
	containers: HashMap<i32, Option<Vec<PawnValue>>>,
	id_allocator: IdAllocator
}

impl PawnVecs {
	pub fn new() -> PawnVecs {
		Self {
			containers: std::collections::HashMap::new(),
			id_allocator: IdAllocator::new()
		}
	}

	pub fn add_container(&mut self) -> i32 {
		let id = self.id_allocator.get_id();
		
		self.containers.insert(id, Some(Vec::new()));

		id
	}
	pub fn remove_container(&mut self, id: i32) -> Result<(), ()> {
		if !self.containers.remove(&id).is_none() {
			Ok(())
		} else {
			Err(())
		}
	}
	pub fn get_mut_container(&mut self, id: i32) -> Result<&mut Vec<PawnValue>, ()> {
		if let Some(container) = self.containers.get_mut(&id).unwrap() {
			Ok(container)
		} else {
			Err(())
		}
	}
}

pub struct PawnAmxVecs(HashMap<AmxIdent, PawnVecs>);

impl PawnAmxVecs {
	
}
