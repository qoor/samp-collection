use samp::prelude::*;
use samp::native;

use crate::plugin::SampCollection;
use crate::collection::*;

impl SampCollection {
	#[native(name = "vec_new")]
	pub fn vec_new(&mut self, amx: &Amx) -> AmxResult<i32> {
		if let Some(vecs) = self.pawn_vecs.get_mut_container_list(&amx).ok() {
			Ok(vecs.add_container(Container::PawnVec(Vec::new())))
		} else {
			Ok(-1)
		}
	}
	#[native(name = "vec_with_capacity")]
	pub fn vec_with_capacity(&mut self, amx: &Amx, capacity: i32) -> AmxResult<i32> {
		if let Some(vecs) = self.pawn_vecs.get_mut_container_list(&amx).ok() {
			Ok(vecs.add_container(Container::PawnVec(Vec::with_capacity(capacity as usize))))
		} else {
			Ok(-1)
		}
	}

	#[native(name = "vec_drop")]
	pub fn vec_drop(&mut self, amx: &Amx, id: i32) -> AmxResult<i32> {
		if let Some(vecs) = self.pawn_vecs.get_mut_container_list(&amx).ok() {
			if vecs.remove_container(id).is_ok() {
				return Ok(1)
			}
		}

		Ok(0)
	}

	#[native(name = "vec_capacity")]
	pub fn vec_capacity(&self, amx: &Amx, id: i32) -> AmxResult<i32> {
		if let Some(container) = get_container(self, &amx, id).ok() {
			Ok(container.capacity() as i32)
		} else {
			Ok(0)
		}
	}
}

fn get_container<'a>(plugin: &'a SampCollection, amx: &Amx, id: i32) -> Result<&'a Vec<crate::value::PawnValue>, ()> {
	if let Some(container) = plugin.pawn_vecs.get_container(&amx, id).ok() {
		match container {
			Container::PawnVec(container) => Ok(container),
			_ => Err(())
		}
	} else {
		Err(())
	}
}

fn get_mut_container<'a>(plugin: &'a mut SampCollection, amx: &Amx, id: i32) -> Result<&'a mut Vec<crate::value::PawnValue>, ()> {
	if let Some(container) = plugin.pawn_vecs.get_mut_container(&amx, id).ok() {
		match container {
			Container::PawnVec(container) => Ok(container),
			_ => Err(())
		}
	} else {
		Err(())
	}
}