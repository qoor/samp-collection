use std::rc::Rc;
use std::cell::RefCell;

use samp::prelude::*;
use samp::native;

use crate::plugin::SampCollection;
use crate::value::*;

impl SampCollection<'static> {
	#[native(name = "vec_new")]
	pub fn vec_new(&mut self, amx: &Amx) -> AmxResult<i32> {
		if let Some(vecs) = self.pawn_vecs.get_mut_container_list(amx) {
			Ok(vecs.add_container(Vec::new()))
		} else {
			Ok(-1)
		}
	}
	#[native(name = "vec_with_capacity")]
	pub fn vec_with_capacity(&mut self, amx: &Amx, capacity: i32) -> AmxResult<i32> {
		if let Some(vecs) = self.pawn_vecs.get_mut_container_list(amx) {
			Ok(vecs.add_container(Vec::with_capacity(capacity as usize)))
		} else {
			Ok(-1)
		}
	}

	#[native(name = "vec_drop")]
	pub fn vec_drop(&mut self, amx: &Amx, id: i32) -> AmxResult<i32> {
		if let Some(vecs) = self.pawn_vecs.get_mut_container_list(&amx) {
			if vecs.remove_container(id).is_ok() {
				return Ok(1)
			}
		}

		Ok(0)
	}

	#[native(name = "vec_capacity")]
	pub fn vec_capacity(&self, amx: &Amx, id: i32) -> AmxResult<i32> {
		if let Some(container) = get_container(self, amx, id) {
			Ok(container.capacity() as i32)
		} else {
			Ok(0)
		}
	}

	#[native(name = "vec_reserve")]
	pub fn vec_reserve(&mut self, amx: &Amx, id: i32, additional: i32) -> AmxResult<i32> {
		if let Some(container) = get_mut_container(self, amx, id) {
			container.reserve(additional as usize);

			Ok(1)
		} else {
			Ok(0)
		}
	}

	#[native(name = "vec_reserve_exact")]
	pub fn vec_reserve_exact(&mut self, amx: &Amx, id: i32, additional: i32) -> AmxResult<i32> {
		if let Some(container) = get_mut_container(self, amx, id) {
			container.reserve_exact(additional as usize);

			Ok(1)
		} else {
			Ok(0)
		}
	}

	#[native(name = "vec_shrink_to_fit")]
	pub fn vec_shrink_to_fit(&mut self, amx: &Amx, id: i32) -> AmxResult<i32> {
		if let Some(container) = get_mut_container(self, amx, id) {
			container.shrink_to_fit();

			Ok(1)
		} else {
			Ok(0)
		}
	}

	#[native(name = "vec_truncate")]
	pub fn vec_truncate(&mut self, amx: &Amx, id: i32, len: i32) -> AmxResult<i32> {
		if let Some(container) = get_mut_container(self, amx, id) {
			container.truncate(len as usize);

			Ok(1)
		} else {
			Ok(0)
		}
	}

	#[native(name = "vec_unsafe_set_len")]
	pub fn vec_unsafe_set_len(&mut self, amx: &Amx, id: i32, new_size: i32) -> AmxResult<i32> {
		if let Some(container) = get_mut_container(self, amx, id) {
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
		if let Some(container) = get_mut_container(self, amx, id) {
			container.swap_remove(index as usize);

			Ok(1)
		} else {
			Ok(0)
		}
	}

	#[native(name = "vec_insert_int")]
	pub fn vec_insert_int(&mut self, amx: &Amx, id: i32, index: i32, value: i32) -> AmxResult<i32> {
		if let Some(container) = get_mut_container(self, amx, id) {
			container.insert(index as usize, PawnValue::Integer(value));

			Ok(1)
		} else {
			Ok(0)
		}
	}
	#[native(name = "vec_insert_float")]
	pub fn vec_insert_float(&mut self, amx: &Amx, id: i32, index: i32, value: f32) -> AmxResult<i32> {
		if let Some(container) = get_mut_container(self, amx, id) {
			container.insert(index as usize, PawnValue::Float(value));

			Ok(1)
		} else {
			Ok(0)
		}
	}
	#[native(name = "vec_insert_array")]
	pub fn vec_insert_array(&mut self, amx: &Amx, id: i32, index: i32, array: UnsizedBuffer, size: i32) -> AmxResult<i32> {
		if let Some(container) = get_mut_container(self, amx, id) {
			let array = array.into_sized_buffer(size as usize).as_slice().to_vec();

			container.insert(index as usize, PawnValue::Array(array));

			Ok(1)
		} else {
			Ok(0)
		}
	}

	#[native(name = "vec_remove_int")]
	pub fn vec_remove_int(&mut self, amx: &Amx, id: i32, index: i32, mut destination: Ref<i32>) -> AmxResult<i32>{
		if let Some(container) = get_mut_container(self, amx, id) {
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
		if let Some(container) = get_mut_container(self, amx, id) {
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
		if let Some(container) = get_mut_container(self, amx, id) {
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
		if let Some(container) = get_mut_container(self, amx, id) {
			container.push(PawnValue::Integer(value));

			Ok(1)
		} else {
			Ok(0)
		}
	}
	#[native(name = "vec_push_float")]
	pub fn vec_push_float(&mut self, amx: &Amx, id: i32, value: f32) -> AmxResult<i32> {
		if let Some(container) = get_mut_container(self, amx, id) {
			container.push(PawnValue::Float(value));

			Ok(1)
		} else {
			Ok(0)
		}
	}
	#[native(name = "vec_push_array")]
	pub fn vec_push_array(&mut self, amx: &Amx, id: i32, array: UnsizedBuffer, size: i32) -> AmxResult<i32> {
		if let Some(container) = get_mut_container(self, amx, id) {
			let array = array.into_sized_buffer(size as usize).as_slice().to_vec();

			container.push(PawnValue::Array(array));

			Ok(1)
		} else {
			Ok(0)
		}
	}

	#[native(name = "vec_pop_int")]
	pub fn vec_pop_int(&mut self, amx: &Amx, id: i32, mut destination: Ref<i32>) -> AmxResult<i32> {
		if let Some(container) = get_mut_container(self, amx, id) {
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
		if let Some(container) = get_mut_container(self, amx, id) {
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
		if let Some(container) = get_mut_container(self, amx, id) {
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
		if let Some(container) = get_mut_container(self, amx, id) {
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

	#[native(name = "vec_append")]
	pub fn vec_append(&mut self, amx: &Amx, source_id: i32, dest_id: i32) -> AmxResult<i32> {
		if source_id != dest_id {
			let borrowed_self = Rc::new(RefCell::from(self));

			if let Some(source_container) = get_mut_container(&mut Rc::clone(&borrowed_self).borrow_mut(), amx, source_id) {
				if let Some(dest_container) = get_mut_container(&mut Rc::clone(&borrowed_self).borrow_mut(), amx, dest_id) {
					dest_container.append(source_container);

					return Ok(1);
				}
			}
		}

		Ok(0)
	}

	#[native(name = "vec_drain")]
	pub fn vec_drain(&mut self, amx: &Amx, source_id: i32, mut new_id: Ref<i32>, start: i32, end: i32) -> AmxResult<i32> {
		if let Some(container_list) = self.pawn_vecs.get_mut_container_list(amx) {
			if let Some(source_container) = container_list.get_mut_container(source_id) {
				let new_container: Vec<PawnValue>;
				
				if start == -1 {
					if end == -1 {
						new_container = source_container.drain(..).collect();
					} else {
						new_container = source_container.drain(..end as usize).collect();
					}
				} else {
					if end == -1 {
						new_container = source_container.drain(start as usize..).collect();
					} else {
						new_container = source_container.drain(start as usize..end as usize).collect();
					}
				}

				*new_id = container_list.add_container(new_container);

				return Ok(1);
			}

		}

		Ok(0)
	}

	#[native(name = "vec_clear")]
	pub fn vec_clear(&mut self, amx: &Amx, id: i32) -> AmxResult<i32> {
		if let Some(container) = get_mut_container(self, amx, id) {
			container.clear();

			Ok(1)
		} else {
			Ok(0)
		}
	}

	#[native(name = "vec_len")]
	pub fn vec_len(&mut self, amx: &Amx, id: i32) -> AmxResult<i32> {
		if let Some(container) = get_container(self, amx, id) {
			Ok(container.len() as i32)
		} else {
			Ok(0)
		}
	}

	#[native(name = "vec_split_off")]
	pub fn vec_split_off(&mut self, amx: &Amx, id: i32, mut new_id: Ref<i32>, at: i32) -> AmxResult<i32> {
		if let Some(container_list) = self.pawn_vecs.get_mut_container_list(amx) {
			if let Some(container) = container_list.get_mut_container(id) {
				let new_container = container.split_off(at as usize);

				*new_id = container_list.add_container(new_container);

				return Ok(1);
			}
		}

		Ok(0)
	}

	#[native(name = "vec_resize_int")]
	pub fn vec_resize_int(&mut self, amx: &Amx, id: i32, new_len: i32, value: i32) -> AmxResult<i32> {
		if let Some(container) = get_mut_container(self, amx, id) {
			container.resize(new_len as usize, PawnValue::Integer(value));

			Ok(1)
		} else {
			Ok(0)
		}
	}
	#[native(name = "vec_resize_float")]
	pub fn vec_resize_float(&mut self, amx: &Amx, id: i32, new_len: i32, value: f32) -> AmxResult<i32> {
		if let Some(container) = get_mut_container(self, amx, id) {
			container.resize(new_len as usize, PawnValue::Float(value));

			Ok(1)
		} else {
			Ok(0)
		}
	}
	#[native(name = "vec_resize_array")]
	pub fn vec_resize_array(&mut self, amx: &Amx, id: i32, new_len: i32, source_array: UnsizedBuffer, size: i32) -> AmxResult<i32> {
		if let Some(container) = get_mut_container(self, amx, id) {
			let buffer = source_array.into_sized_buffer(size as usize).to_vec();

			container.resize(new_len as usize, PawnValue::Array(buffer));

			Ok(1)
		} else {
			Ok(0)
		}
	}

	#[native(name = "vec_dedup")]
	pub fn vec_dedup(&mut self, amx: &Amx, id: i32) -> AmxResult<i32> {
		if let Some(container) = get_mut_container(self, amx, id) {
			container.dedup();

			Ok(1)
		} else {
			Ok(0)
		}
	}

	#[native(name = "vec_is_empty")]
	pub fn vec_is_empty(&mut self, amx: &Amx, id: i32) -> AmxResult<i32> {
		if let Some(container) = get_container(self, amx, id) {
			Ok(container.is_empty() as i32)
		} else {
			Ok(0)
		}
	}

	#[native(name = "vec_get_int")]
	pub fn vec_get_int(&mut self, amx: &Amx, id: i32, index: i32, mut destination: Ref<i32>) -> AmxResult<i32> {
		if let Some(container) = get_container(self, amx, id) {
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
		if let Some(container) = get_container(self, amx, id) {
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
		if let Some(container) = get_container(self, amx, id) {
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
		if let Some(container) = get_container(self, amx, id) {
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
		if let Some(container) = get_container(self, amx, id) {
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
		if let Some(container) = get_container(self, amx, id) {
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

	#[native(name = "vec_contains_int")]
	pub fn vec_contains_int(&self, amx: &Amx, id: i32, value: i32) -> AmxResult<i32> {
		if let Some(container) = get_container(self, amx, id) {
			Ok(container.contains(&PawnValue::Integer(value)) as i32)
		} else {
			Ok(0)
		}
	}
	#[native(name = "vec_contains_float")]
	pub fn vec_contains_float(&self, amx: &Amx, id: i32, value: f32) -> AmxResult<i32> {
		if let Some(container) = get_container(self, amx, id) {
			Ok(container.contains(&PawnValue::Float(value)) as i32)
		} else {
			Ok(0)
		}
	}
	#[native(name = "vec_contains_array")]
	pub fn vec_contains_array(&self, amx: &Amx, id: i32, value: UnsizedBuffer, size: i32) -> AmxResult<i32> {
		if let Some(container) = get_container(self, amx, id) {
			let buffer = value.into_sized_buffer(size as usize).as_slice().to_vec();

			Ok(container.contains(&PawnValue::Array(buffer)) as i32)
		} else {
			Ok(0)
		}
	}

	#[native(name = "vec_binary_search_int")]
	pub fn vec_binary_search_int(&self, amx: &Amx, id: i32, value: i32) -> AmxResult<i32> {
		if let Some(container) = get_container(self, amx, id) {
			let found = container.binary_search(&PawnValue::Integer(value));

			return if found.is_ok() {
				Ok(found.ok().unwrap() as i32)
			} else {
				Ok(found.err().unwrap() as i32)
			};
		}

		Ok(-1)
	}
	#[native(name = "vec_binary_search_float")]
	pub fn vec_binary_search_float(&self, amx: &Amx, id: i32, value: f32) -> AmxResult<i32> {
		if let Some(container) = get_container(self, amx, id) {
			let found = container.binary_search(&PawnValue::Float(value));

			return if found.is_ok() {
				Ok(found.ok().unwrap() as i32)
			} else {
				Ok(found.err().unwrap() as i32)
			};
		}

		Ok(-1)
	}
	#[native(name = "vec_binary_search_array")]
	pub fn vec_binary_search_array(&self, amx: &Amx, id: i32, value: UnsizedBuffer, size: i32) -> AmxResult<i32> {
		if let Some(container) = get_container(self, amx, id) {
			let buffer = value.into_sized_buffer(size as usize).as_slice().to_vec();
			let found = container.binary_search(&PawnValue::Array(buffer));

			return if found.is_ok() {
				Ok(found.ok().unwrap() as i32)
			} else {
				Ok(found.err().unwrap() as i32)
			};
		}

		Ok(-1)
	}

	#[native(name = "vec_sort_unstable")]
	pub fn vec_sort_unstable(&mut self, amx: &Amx, id: i32) -> AmxResult<i32> {
		if let Some(container) = get_mut_container(self, amx, id) {
			container.sort_unstable();

			Ok(1)
		} else {
			Ok(0)
		}
	}
	#[native(name = "vec_rsort_unstable")]
	pub fn vec_rsort_unstable(&mut self, amx: &Amx, id: i32) -> AmxResult<i32> {
		if let Some(container) = get_mut_container(self, amx, id) {
			container.sort_unstable_by(|a, b| b.cmp(a));

			Ok(1)
		} else {
			Ok(0)
		}
	}

	#[native(name = "vec_split_at")]
	pub fn vec_split_at(&mut self, amx: &Amx, id: i32, mut new_left_id: Ref<i32>, mut new_right_id: Ref<i32>) -> AmxResult<i32> {
		if let Some(container_list) = self.pawn_vecs.get_mut_container_list(amx) {
			let borrowed_list = Rc::new(RefCell::from(container_list));

			if let Some(container) = Rc::clone(&borrowed_list).borrow_mut().get_mut_container(id) {
				let (left, right) = container.split_at(id as usize);
				let borrowed_list = Rc::clone(&borrowed_list);

				*new_left_id = borrowed_list.borrow_mut().add_container(left.to_vec());
				*new_right_id = borrowed_list.borrow_mut().add_container(right.to_vec());

				return Ok(1);
			}
		}
		
		Ok(0)
	}

	#[native(name = "vec_rotate_left")]
	pub fn vec_rotate_left(&mut self, amx: &Amx, id: i32, mid: i32) -> AmxResult<i32> {
		if let Some(container) = get_mut_container(self, amx, id) {
			container.rotate_left(mid as usize);

			Ok(1)
		} else {
			Ok(0)
		}
	}

	#[native(name = "vec_rotate_right")]
	pub fn vec_rotate_right(&mut self, amx: &Amx, id: i32, k: i32) -> AmxResult<i32> {
		if let Some(container) = get_mut_container(self, amx, id) {
			container.rotate_right(k as usize);

			Ok(1)
		} else {
			Ok(0)
		}
	}

	#[native(name = "vec_sort")]
	pub fn vec_sort(&mut self, amx: &Amx, id: i32) -> AmxResult<i32> {
		if let Some(container) = get_mut_container(self, amx, id) {
			container.sort();

			Ok(1)
		} else {
			Ok(0)
		}
	}

	#[native(name = "vec_clone")]
	pub fn vec_clone(&mut self, amx: &Amx, id: i32, mut new_id: Ref<i32>) -> AmxResult<i32> {
		let borrowed_self = Rc::new(RefCell::from(self));

		if let Some(container_list) = Rc::clone(&borrowed_self).borrow_mut().pawn_vecs.get_mut_container_list(amx) {
			if let Some(container) = get_container(&Rc::clone(&borrowed_self).borrow(), amx, id) {
				let new_container = container.clone();

				*new_id = container_list.add_container(new_container);

				return Ok(1);
			}
		}

		Ok(0)
	}

	#[native(name = "vec_concat")]
	pub fn vec_concat(&mut self, amx: &Amx, id1: i32, id2: i32, mut new_id: Ref<i32>) -> AmxResult<i32> {
		let borrowed_self = Rc::new(RefCell::from(self));

		if let Some(container_list) = Rc::clone(&borrowed_self).borrow_mut().pawn_vecs.get_mut_container_list(amx) {
			if let Some(container1) = get_container(&Rc::clone(&borrowed_self).borrow(), amx, id1) {
				if let Some(container2) = get_container(&Rc::clone(&borrowed_self).borrow(), amx, id2) {
					let new_container = [container1.as_slice(), container2.as_slice()].concat();

					*new_id = container_list.add_container(new_container);

					return Ok(1);
				}
			}
		}

		Ok(0)
	}
}

fn get_container<'a>(plugin: &'a SampCollection, amx: &Amx, id: i32) -> Option<&'a Vec<crate::value::PawnValue>> {
	if let Some(container) = plugin.pawn_vecs.get_container(&amx, id) {
		Some(container)
	} else {
		None
	}
}

fn get_mut_container<'a>(plugin: &'a mut SampCollection, amx: &Amx, id: i32) -> Option<&'a mut Vec<crate::value::PawnValue>> {
	if let Some(container) = plugin.pawn_vecs.get_mut_container(&amx, id) {
		Some(container)
	} else {
		None
	}
}