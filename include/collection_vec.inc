//====================================================
//				SA-MP collection plugin
//			Rust std::collections for SA-MP
//
//				version: v0.0.1 alpha
//
//		Copyright(c) 2019 Qoo. All rights reserved.
//====================================================

#define INVALID_VEC_ID 0xFFFFFFFF

native vec_new();
native vec_with_capacity(capacity);
native vec_drop(vecid);
native vec_capacity(vecid);
native vec_reserve(vecid, additional);
native vec_reserve_excat(vecid, additional);
native vec_shrink_to_fit(vecid);
native vec_truncate(vecid);
native vec_unsafe_set_len(vecid, new_size);
native vec_swap_remove(vecid, index);
native vec_insert_int(vecid, index, value);
native vec_insert_float(vecid, index, Float: value);
native vec_insert_array(vecid, index, const array[], size = sizeof(array));
native vec_remove_int(vecid, index, &destination);
native vec_remove_float(vecid, index, &Float: destination);
native vec_remove_array(vecid, index, destArray[], size = sizeof(array));
native vec_push_int(vecid, value);
native vec_push_float(vecid, Float: value);
native vec_push_array(vecid, const array[], size = sizeof(array));
native vec_pop_int(vecid, &destination);
native vec_pop_float(vecid, &Float: destination);
native vec_pop_array(vecid, destArray[], size = sizeof(array));
native vec_get_type(vecid, index);
native vec_drain(vecid, &newVecId, start = -1, end = -1);
native vec_clear(vecid);
native vec_len(vecid);
native vec_split_off(vecid, &newVecId, at);
