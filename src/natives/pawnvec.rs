use std::rc::Rc;
use std::cell::{RefCell, RefMut};

use samp::prelude::*;
use samp::native;
use samp::cell::buffer::*;

use crate::plugin::SampCollection;
use crate::collection::*;
use crate::value::*;

impl SampCollection {
	#[native(name = "vec_new")]
	pub fn vec_new(&mut self, amx: &Amx) -> AmxResult<i32> {
		if let Some(vecs) = self.pawn_vecs.get_mut_container_list(amx).ok() {
			Ok(vecs.add_container(Container::PawnVec(Vec::new())))
		} else {
			Ok(-1)
		}
	}
	#[native(name = "vec_with_capacity")]
	pub fn vec_with_capacity(&mut self, amx: &Amx, capacity: i32) -> AmxResult<i32> {
		if let Some(vecs) = self.pawn_vecs.get_mut_container_list(amx).ok() {
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
		if let Some(container) = get_container(self, amx, id).ok() {
			Ok(container.capacity() as i32)
		} else {
			Ok(0)
		}
	}

	#[native(name = "vec_reserve")]
	pub fn vec_reserve(&mut self, amx: &Amx, id: i32, additional: i32) -> AmxResult<i32> {
		if let Some(container) = get_mut_container(self, amx, id).ok() {
			container.reserve(additional as usize);

			Ok(1)
		} else {
			Ok(0)
		}
	}

	#[native(name = "vec_reserve_exact")]
	pub fn vec_reserve_exact(&mut self, amx: &Amx, id: i32, additional: i32) -> AmxResult<i32> {
		if let Some(container) = get_mut_container(self, amx, id).ok() {
			container.reserve_exact(additional as usize);

			Ok(1)
		} else {
			Ok(0)
		}
	}

	#[native(name = "vec_shrink_to_fit")]
	pub fn vec_shrink_to_fit(&mut self, amx: &Amx, id: i32) -> AmxResult<i32> {
		if let Some(container) = get_mut_container(self, amx, id).ok() {
			container.shrink_to_fit();

			Ok(1)
		} else {
			Ok(0)
		}
	}

	#[native(name = "vec_truncate")]
	pub fn vec_truncate(&mut self, amx: &Amx, id: i32, len: i32) -> AmxResult<i32> {
		if let Some(container) = get_mut_container(self, amx, id).ok() {
			container.truncate(len as usize);

			Ok(1)
		} else {
			Ok(0)
		}
	}

	#[native(name = "vec_unsafe_set_len")]
	pub fn vec_unsafe_set_len(&mut self, amx: &Amx, id: i32, new_size: i32) -> AmxResult<i32> {
		if let Some(container) = get_mut_container(self, amx, id).ok() {
			unsafe {
				container.set_len(new_size as usize);
			}
			
			Ok(1)
		} else {
			Ok(0)
		}
	}

	#[native(name = "vec_swap_remove")]
	pub fn vec_swap_remove(&mut self, amx: &Amx, id: i32, index: i32) -> AmxResult<i32> {
		if let Some(container) = get_mut_container(self, amx, id).ok() {
			container.swap_remove(index as usize);

			Ok(1)
		} else {
			Ok(0)
		}
	}

	#[native(name = "vec_insert_int")]
	pub fn vec_insert_int(&mut self, amx: &Amx, id: i32, index: i32, value: i32) -> AmxResult<i32> {
		if let Some(container) = get_mut_container(self, amx, id).ok() {
			container.insert(index as usize, PawnValue::Integer(value));

			Ok(1)
		} else {
			Ok(0)
		}
	}
	#[native(name = "vec_insert_float")]
	pub fn vec_insert_float(&mut self, amx: &Amx, id: i32, index: i32, value: f32) -> AmxResult<i32> {
		if let Some(container) = get_mut_container(self, amx, id).ok() {
			container.insert(index as usize, PawnValue::Float(value));

			Ok(1)
		} else {
			Ok(0)
		}
	}
	#[native(name = "vec_insert_array")]
	pub fn vec_insert_array(&mut self, amx: &Amx, id: i32, index: i32, array: UnsizedBuffer, size: i32) -> AmxResult<i32> {
		if let Some(container) = get_mut_container(self, amx, id).ok() {
			let array = array.into_sized_buffer(size as usize).as_slice().to_vec();

			container.insert(index as usize, PawnValue::Array(PawnArray::new(array)));

			Ok(1)
		} else {
			Ok(0)
		}
	}

	#[native(name = "vec_remove_int")]
	pub fn vec_remove_int(&mut self, amx: &Amx, id: i32, index: i32, mut destination: Ref<i32>) -> AmxResult<i32>{
		if let Some(container) = get_mut_container(self, amx, id).ok() {
			match container.remove(index as usize) {
				PawnValue::Integer(value) => {
					*destination = value;

					Ok(1)
				},
				_ => Ok(-1)
			}
		} else {
			Ok(0)
		}
	}
	#[native(name = "vec_remove_float")]
	pub fn vec_remove_float(&mut self, amx: &Amx, id: i32, index: i32, mut destination: Ref<f32>) -> AmxResult<i32>{
		if let Some(container) = get_mut_container(self, amx, id).ok() {
			match container.remove(index as usize) {
				PawnValue::Float(value) => {
					*destination = value;

					Ok(1)
				},
				_ => Ok(-1)
			}
		} else {
			Ok(0)
		}
	}
	#[native(name = "vec_remove_array")]
	pub fn vec_remove_array(&mut self, amx: &Amx, id: i32, index: i32, mut destination: UnsizedBuffer, size: i32) -> AmxResult<i32>{
		if let Some(container) = get_mut_container(self, amx, id).ok() {
			match container.remove(index as usize) {
				PawnValue::Array(value) => {
					let buffer = destination.as_mut_ptr();
					let mut fixed_size = value.len() as i32;

					if fixed_size > size {
						fixed_size = size;
					}

					unsafe {
						std::ptr::copy(value.as_ptr(), buffer, fixed_size as usize);
					}

					Ok(1)
				},
				_ => Ok(-1)
			}
		} else {
			Ok(0)
		}
	}

	#[native(name = "vec_push_int")]
	pub fn vec_push_int(&mut self, amx: &Amx, id: i32, value: i32) -> AmxResult<i32> {
		if let Some(container) = get_mut_container(self, amx, id).ok() {
			container.push(PawnValue::Integer(value));

			Ok(1)
		} else {
			Ok(0)
		}
	}
	#[native(name = "vec_push_float")]
	pub fn vec_push_float(&mut self, amx: &Amx, id: i32, value: f32) -> AmxResult<i32> {
		if let Some(container) = get_mut_container(self, amx, id).ok() {
			container.push(PawnValue::Float(value));

			Ok(1)
		} else {
			Ok(0)
		}
	}
	#[native(name = "vec_push_array")]
	pub fn vec_push_array(&mut self, amx: &Amx, id: i32, array: UnsizedBuffer, size: i32) -> AmxResult<i32> {
		if let Some(container) = get_mut_container(self, amx, id).ok() {
			let array = array.into_sized_buffer(size as usize).as_slice().to_vec();

			container.push(PawnValue::Array(PawnArray::new(array)));

			Ok(1)
		} else {
			Ok(0)
		}
	}

	#[native(name = "vec_pop_int")]
	pub fn vec_pop_int(&mut self, amx: &Amx, id: i32, mut destination: Ref<i32>) -> AmxResult<i32> {
		if let Some(container) = get_mut_container(self, amx, id).ok() {
			match container.pop().unwrap() {
				PawnValue::Integer(value) => {
					*destination = value;

					Ok(1)
				}
				_ => Ok(-1)
			}
		} else {
			Ok(0)
		}
	}
	#[native(name = "vec_pop_float")]
	pub fn vec_pop_float(&mut self, amx: &Amx, id: i32, mut destination: Ref<f32>) -> AmxResult<i32> {
		if let Some(container) = get_mut_container(self, amx, id).ok() {
			match container.pop().unwrap() {
				PawnValue::Float(value) => {
					*destination = value;

					Ok(1)
				}
				_ => Ok(-1)
			}
		} else {
			Ok(0)
		}
	}
	#[native(name = "vec_pop_array")]
	pub fn vec_pop_array(&mut self, amx: &Amx, id: i32, mut destination: UnsizedBuffer, size: i32) -> AmxResult<i32> {
		if let Some(container) = get_mut_container(self, amx, id).ok() {
			match container.pop().unwrap() {
				PawnValue::Array(value) => {
					let buffer = destination.as_mut_ptr();
					let mut fixed_size = value.len() as i32;
					
					if fixed_size > size {
						fixed_size = size;
					}
					
					unsafe {
						std::ptr::copy(value.as_ptr(), buffer, fixed_size as usize);
					}

					Ok(1)
				},
				_ => Ok(-1)
			}
		} else {
			Ok(0)
		}
	}

	#[native(name = "vec_get_type")]
	pub fn vec_get_type(&mut self, amx: &Amx, id: i32, index: i32) -> AmxResult<i32> {
		if let Some(container) = get_mut_container(self, amx, id).ok() {
			if let Some(value) = container.get(index as usize) {
				return match value {
					PawnValue::Integer(_) => Ok(PawnExportValueType::Integer as i32),
					PawnValue::Float(_) => Ok(PawnExportValueType::Float as i32),
					PawnValue::Array(_) => Ok(PawnExportValueType::Array as i32)
				};
			}
		}
		
		Ok(0)
	}

	/*#[native(name = "vec_append")]
	pub fn vec_append(&mut self, amx: &Amx, dest_id: i32, source_id: i32) -> AmxResult<i32> {
		if let Some(source_container) = get_mut_container(self, amx, source_id).ok() {
			if let Some(dest_container) = get_mut_container(self, amx, dest_id).ok() {
				dest_container.append(source_container);

				return Ok(1);
			}
		}

		Ok(0)
	}*/

	#[native(name = "vec_drain")]
	pub fn vec_drain(&mut self, amx: &Amx, source_id: i32, mut new_id: Ref<i32>, start: i32, end: i32) -> AmxResult<i32> {
		if let Some(container_list) = self.pawn_vecs.get_mut_container_list(amx).ok() {
			if let Some(source_container) = container_list.get_mut_container(source_id).ok() {
				if let Container::PawnVec(source_container) = source_container {
					let new_container = source_container.drain((start as usize)..(end as usize)).collect();

					*new_id = container_list.add_container(Container::PawnVec(new_container));

					return Ok(1);
				}
			}

		}

		Ok(0)
	}

	#[native(name = "vec_clear")]
	pub fn vec_clear(&mut self, amx: &Amx, id: i32) -> AmxResult<i32> {
		if let Some(container) = get_mut_container(self, amx, id).ok() {
			container.clear();

			Ok(1)
		} else {
			Ok(0)
		}
	}

	#[native(name = "vec_len")]
	pub fn vec_len(&mut self, amx: &Amx, id: i32) -> AmxResult<i32> {
		if let Some(container) = get_container(self, amx, id).ok() {
			Ok(container.len() as i32)
		} else {
			Ok(0)
		}
	}

	#[native(name = "vec_split_off")]
	pub fn vec_split_off(&mut self, amx: &Amx, id: i32, mut new_id: Ref<i32>, at: i32) -> AmxResult<i32> {
		if let Some(container_list) = self.pawn_vecs.get_mut_container_list(amx).ok() {
			if let Some(container) = container_list.get_mut_container(id).ok() {
				if let Container::PawnVec(container) = container {
					let new_container = container.split_off(at as usize);

					*new_id = container_list.add_container(Container::PawnVec(new_container));

					return Ok(1);
				}
			}
		}

		Ok(0)
	}

	#[native(name = "vec_resize_int")]
	pub fn vec_resize_int(&mut self, amx: &Amx, id: i32, new_len: i32, value: i32) -> AmxResult<i32> {
		if let Some(container) = get_mut_container(self, amx, id).ok() {
			container.resize(new_len as usize, PawnValue::Integer(value));

			Ok(1)
		} else {
			Ok(0)
		}
	}
	#[native(name = "vec_resize_float")]
	pub fn vec_resize_float(&mut self, amx: &Amx, id: i32, new_len: i32, value: f32) -> AmxResult<i32> {
		if let Some(container) = get_mut_container(self, amx, id).ok() {
			container.resize(new_len as usize, PawnValue::Float(value));

			Ok(1)
		} else {
			Ok(0)
		}
	}
	#[native(name = "vec_resize_array")]
	pub fn vec_resize_array(&mut self, amx: &Amx, id: i32, new_len: i32, source_array: UnsizedBuffer, size: i32) -> AmxResult<i32> {
		if let Some(container) = get_mut_container(self, amx, id).ok() {
			let buffer = source_array.into_sized_buffer(size as usize).to_vec();

			container.resize(new_len as usize, PawnValue::Array(PawnArray::new(buffer)));

			Ok(1)
		} else {
			Ok(0)
		}
	}

	#[native(name = "vec_dedup")]
	pub fn vec_dedup(&mut self, amx: &Amx, id: i32) -> AmxResult<i32> {
		if let Some(container) = get_mut_container(self, amx, id).ok() {
			container.dedup();

			Ok(1)
		} else {
			Ok(0)
		}
	}

	#[native(name = "vec_is_empty")]
	pub fn vec_is_empty(&mut self, amx: &Amx, id: i32) -> AmxResult<i32> {
		if let Some(container) = get_container(self, amx, id).ok() {
			Ok(container.is_empty() as i32)
		} else {
			Ok(0)
		}
	}

	#[native(name = "vec_get_int")]
	pub fn vec_get_int(&mut self, amx: &Amx, id: i32, index: i32, mut destination: Ref<i32>) -> AmxResult<i32> {
		if let Some(container) = get_container(self, amx, id).ok() {
			if let Some(value) = container.get(index as usize) {
				if let PawnValue::Integer(value) = value {
					*destination = *value;

					return Ok(1);
				}
			}
		}

		Ok(0)
	}
	#[native(name = "vec_get_float")]
	pub fn vec_get_float(&mut self, amx: &Amx, id: i32, index: i32, mut destination: Ref<f32>) -> AmxResult<i32> {
		if let Some(container) = get_container(self, amx, id).ok() {
			if let Some(value) = container.get(index as usize) {
				if let PawnValue::Float(value) = value {
					*destination = *value;

					return Ok(1);
				}
			}
		}

		Ok(0)
	}
	#[native(name = "vec_get_array")]
	pub fn vec_get_array(&mut self, amx: &Amx, id: i32, index: i32, mut destination: UnsizedBuffer, size: i32) -> AmxResult<i32> {
		if let Some(container) = get_container(self, amx, id).ok() {
			if let Some(value) = container.get(index as usize) {
				if let PawnValue::Array(value) = value {
					let buffer = destination.as_mut_ptr();
					let mut fixed_size = value.len() as i32;

					if fixed_size > size {
						fixed_size = size;
					}

					unsafe {
						std::ptr::copy(value.as_ptr(), buffer, fixed_size as usize);
					}

					return Ok(1);
				}
			}
		}

		Ok(0)
	}

	#[native(name = "vec_first_int")]
	pub fn vec_first_int(&mut self, amx: &Amx, id: i32, destination: Ref<i32>) -> AmxResult<i32> {
		self.vec_get_int(amx, id, 0, destination)
	}
	#[native(name = "vec_first_float")]
	pub fn vec_first_float(&mut self, amx: &Amx, id: i32, destination: Ref<f32>) -> AmxResult<i32> {
		self.vec_get_float(amx, id, 0, destination)
	}
	#[native(name = "vec_first_array")]
	pub fn vec_first_array(&mut self, amx: &Amx, id: i32, destination: UnsizedBuffer, size: i32) -> AmxResult<i32> {
		self.vec_get_array(amx, id, 0, destination, size)
	}

	#[native(name = "vec_last_int")]
	pub fn vec_last_int(&mut self, amx: &Amx, id: i32, mut destination: Ref<i32>) -> AmxResult<i32> {
		if let Some(container) = get_container(self, amx, id).ok() {
			if let Some(value) = container.last() {
				if let PawnValue::Integer(value) = value {
					*destination = *value;

					return Ok(1);
				}
			}
		}

		Ok(0)
	}
	#[native(name = "vec_last_float")]
	pub fn vec_last_float(&mut self, amx: &Amx, id: i32, mut destination: Ref<f32>) -> AmxResult<i32> {
		if let Some(container) = get_container(self, amx, id).ok() {
			if let Some(value) = container.last() {
				if let PawnValue::Float(value) = value {
					*destination = *value;

					return Ok(1);
				}
			}
		}

		Ok(0)
	}
	#[native(name = "vec_last_array")]
	pub fn vec_last_array(&mut self, amx: &Amx, id: i32, mut destination: UnsizedBuffer, size: i32) -> AmxResult<i32> {
		if let Some(container) = get_container(self, amx, id).ok() {
			if let Some(value) = container.last() {
				if let PawnValue::Array(value) = value {
					let buffer = destination.as_mut_ptr();
					let mut fixed_size = value.len() as i32;

					if fixed_size > size {
						fixed_size = size;
					}

					unsafe {
						std::ptr::copy(value.as_ptr(), buffer, fixed_size as usize);
					}

					return Ok(1);
				}
			}
		}

		Ok(0)
	}
}

fn get_container<'a>(plugin: &'a SampCollection, amx: &Amx, id: i32) -> Result<&'a Vec<crate::value::PawnValue>, ()> {
	if let Some(container) = plugin.pawn_vecs.get_container(&amx, id).ok() {
		if let Container::PawnVec(container) = container {
			return Ok(container)
		}
	}

	Err(())
}

fn get_mut_container<'a>(plugin: &'a mut SampCollection, amx: &Amx, id: i32) -> Result<&'a mut Vec<crate::value::PawnValue>, ()> {
	if let Some(container) = plugin.pawn_vecs.get_mut_container(&amx, id).ok() {
		if let Container::PawnVec(container) = container {
			return Ok(container);
		}
	}

	Err(())
}