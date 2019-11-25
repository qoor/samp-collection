use std::collections::HashMap;

use samp::prelude::*;
use samp::native;

use crate::unsafe_copy;
use crate::plugin::*;
use crate::collection::PawnHashMap;
use crate::value::PawnValue;

impl SampCollection<'static> {
	#[native(name = "hashmap_new")]
	pub fn hashmap_new(&mut self, amx: &Amx) -> AmxResult<i32> {
		if let Some(container_list) = self.pawn_hashmaps.get_mut_container_list(amx) {
			Ok(container_list.add_container(HashMap::new()))
		} else {
			Ok(-1)
		}
	}

	#[native(name = "hashmap_with_capacity")]
	pub fn hashmap_with_capacity(&mut self, amx: &Amx, capacity: i32) -> AmxResult<i32> {
		if let Some(container_list) = self.pawn_hashmaps.get_mut_container_list(amx) {
			Ok(container_list.add_container(HashMap::with_capacity(capacity as usize)))
		} else {
			Ok(-1)
		}
	}

	#[native(name = "hashmap_capacity")]
	pub fn hashmap_capacity(&self, amx: &Amx, id: i32) -> AmxResult<i32> {
		if let Some(container) = get_container(self, amx, id) {
			Ok(container.capacity() as i32)
		} else {
			Ok(0)
		}
	}

	#[native(name = "hashmap_len")]
	pub fn hashmap_len(&self, amx: &Amx, id: i32) -> AmxResult<i32> {
		if let Some(container) = get_container(self, amx, id) {
			Ok(container.len() as i32)
		} else {
			Ok(0)
		}
	}

	#[native(name = "hashmap_is_empty")]
	pub fn hashmap_is_empty(&self, amx: &Amx, id: i32) -> AmxResult<i32> {
		if let Some(container) = get_container(self, amx, id) {
			Ok(container.is_empty() as i32)
		} else {
			Ok(1)
		}
	}

	#[native(name = "hashmap_clear")]
	pub fn hashmap_clear(&mut self, amx: &Amx, id: i32) -> AmxResult<i32> {
		if let Some(container) = get_mut_container(self, amx, id) {
			container.clear();

			Ok(1)
		} else {
			Ok(0)
		}
	}

	#[native(name = "hashmap_reserve")]
	pub fn hashmap_reserve(&mut self, amx: &Amx, id: i32, additional: i32) -> AmxResult<i32> {
		if let Some(container) = get_mut_container(self, amx, id) {
			container.reserve(additional as usize);

			Ok(1)
		} else {
			Ok(0)
		}
	}

	#[native(name = "hashmap_shrink_to_fit")]
	pub fn hashmap_shrink_to_fit(&mut self, amx: &Amx, id: i32) -> AmxResult<i32> {
		if let Some(container) = get_mut_container(self, amx, id) {
			container.shrink_to_fit();

			Ok(1)
		} else {
			Ok(0)
		}
	}

	#[native(name = "hashmap_int_get_int")]
	pub fn hashmap_int_get_int(&self, amx: &Amx, id: i32, key: i32, mut destination: Ref<i32>) -> AmxResult<i32> {
		if let Some(container) = get_container(self, amx, id) {
			if let Some(entry) = container.get(&PawnValue::Integer(key)) {
				if let PawnValue::Integer(value) = entry {
					*destination = *value;

					return Ok(1);
				}
			}
		}

		Ok(0)
	}
	#[native(name = "hashmap_int_get_float")]
	pub fn hashmap_int_get_float(&self, amx: &Amx, id: i32, key: i32, mut destination: Ref<f32>) -> AmxResult<i32> {
		if let Some(container) = get_container(self, amx, id) {
			if let Some(entry) = container.get(&PawnValue::Integer(key)) {
				if let PawnValue::Float(value) = entry {
					*destination = *value;

					return Ok(1);
				}
			}
		}

		Ok(0)
	}
	#[native(name = "hashmap_int_get_array")]
	pub fn hashmap_int_get_array(&self, amx: &Amx, id: i32, key: i32, mut destination: UnsizedBuffer, dest_size: i32) -> AmxResult<i32> {
		if let Some(container) = get_container(self, amx, id) {
			if let Some(entry) = container.get(&PawnValue::Integer(key)) {
				if let PawnValue::Array(value) = entry {
					unsafe_copy!(value, destination, dest_size);

					return Ok(1);
				}
			}
		}

		Ok(0)
	}

	#[native(name = "hashmap_float_get_int")]
	pub fn hashmap_float_get_int(&self, amx: &Amx, id: i32, key: f32, mut destination: Ref<i32>) -> AmxResult<i32> {
		if let Some(container) = get_container(self, amx, id) {
			if let Some(entry) = container.get(&PawnValue::Float(key)) {
				if let PawnValue::Integer(value) = entry {
					*destination = *value;

					return Ok(1);
				}
			}
		}

		Ok(0)
	}
	#[native(name = "hashmap_float_get_float")]
	pub fn hashmap_float_get_float(&self, amx: &Amx, id: i32, key: f32, mut destination: Ref<f32>) -> AmxResult<i32> {
		if let Some(container) = get_container(self, amx, id) {
			if let Some(entry) = container.get(&PawnValue::Float(key)) {
				if let PawnValue::Float(value) = entry {
					*destination = *value;

					return Ok(1);
				}
			}
		}

		Ok(0)
	}
	#[native(name = "hashmap_float_get_array")]
	pub fn hashmap_float_get_array(&self, amx: &Amx, id: i32, key: f32, mut destination: UnsizedBuffer, dest_size: i32) -> AmxResult<i32> {
		if let Some(container) = get_container(self, amx, id) {
			if let Some(entry) = container.get(&PawnValue::Float(key)) {
				if let PawnValue::Array(value) = entry {
					unsafe_copy!(value, destination, dest_size);

					return Ok(1);
				}
			}
		}

		Ok(0)
	}

	#[native(name = "hashmap_array_get_int")]
	pub fn hashmap_array_get_int(&self, amx: &Amx, id: i32, key: UnsizedBuffer, mut destination: Ref<i32>, key_size: i32) -> AmxResult<i32> {
		if let Some(container) = get_container(self, amx, id) {
			let key = key.into_sized_buffer(key_size as usize).to_vec();

			if let Some(entry) = container.get(&PawnValue::Array(key)) {
				if let PawnValue::Integer(value) = entry {
					*destination = *value;

					return Ok(1);
				}
			}
		}

		Ok(0)
	}
	#[native(name = "hashmap_array_get_float")]
	pub fn hashmap_array_get_float(&self, amx: &Amx, id: i32, key: UnsizedBuffer, mut destination: Ref<f32>, key_size: i32) -> AmxResult<i32> {
		if let Some(container) = get_container(self, amx, id) {
			let key = key.into_sized_buffer(key_size as usize).to_vec();

			if let Some(entry) = container.get(&PawnValue::Array(key)) {
				if let PawnValue::Float(value) = entry {
					*destination = *value;

					return Ok(1);
				}
			}
		}

		Ok(0)
	}
	#[native(name = "hashmap_array_get_array")]
	pub fn hashmap_array_get_array(&self, amx: &Amx, id: i32, key: UnsizedBuffer, mut destination: UnsizedBuffer, key_size: i32, dest_size: i32) -> AmxResult<i32> {
		if let Some(container) = get_container(self, amx, id) {
			let key = key.into_sized_buffer(key_size as usize).to_vec();

			if let Some(entry) = container.get(&PawnValue::Array(key)) {
				if let PawnValue::Array(value) = entry {
					unsafe_copy!(value, destination, dest_size);

					return Ok(1);
				}
			}
		}

		Ok(0)
	}
}

fn get_container<'a>(plugin: &'a SampCollection, amx: &Amx, id: i32) -> Option<&'a PawnHashMap> {
	plugin.pawn_hashmaps.get_container(&amx, id)
}

fn get_mut_container<'a>(plugin: &'a mut SampCollection, amx: &Amx, id: i32) -> Option<&'a mut PawnHashMap> {
	plugin.pawn_hashmaps.get_mut_container(&amx, id)
}

