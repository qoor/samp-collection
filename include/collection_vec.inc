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
native vec_unsafe_set_len(vecid, newSize);
native vec_swap_remove(vecid, index);
native vec_insert_int(vecid, index, value);
native vec_insert_float(vecid, index, Float: value);
native vec_insert_array(vecid, index, const value[], size = sizeof(value));
native vec_remove_int(vecid, index, &destination);
native vec_remove_float(vecid, index, &Float: destination);
native vec_remove_array(vecid, index, destination[], size = sizeof(destination));
native vec_push_int(vecid, value);
native vec_push_float(vecid, Float: value);
native vec_push_array(vecid, const value[], size = sizeof(value));
native vec_pop_int(vecid, &destination);
native vec_pop_float(vecid, &Float: destination);
native vec_pop_array(vecid, destination[], size = sizeof(destination));
native vec_get_type(vecid, index);
native vec_append(vecid, sourceId, destId);
native vec_drain(vecid, start = -1, end = -1);
native vec_clear(vecid);
native vec_len(vecid);
native vec_split_off(vecid, at);
native vec_resize_int(vecid, newLen, value);
native vec_resize_float(vecid, newLen, Float: value);
native vec_resize_array(vecid, newLen, value[], size = sizeof(value));
native vec_dedup(vecid);
native vec_is_empty(vecid);
native vec_get_int(vecid, index, &destination);
native vec_get_float(vecid, index, &Float: destination);
native vec_get_array(vecid, index, destination[], size = sizeof(destination));
native vec_first_int(vecid, &destination);
native vec_first_float(vecid, &Float: destination);
native vec_first_array(vecid, destination[], size = sizeof(destination));
native vec_last_int(vecid, &destination);
native vec_last_float(vecid, &Float: destination);
native vec_last_array(vecid, destination[], size = sizeof(destination));
native vec_contains_int(vecid, value);
native vec_contains_float(vecid, &Float: value);
native vec_contains_array(vecid, value[], size = sizeof(value));
native vec_binary_search_int(vecid, value);
native vec_binary_search_float(vecid, &Float: value);
native vec_binary_search_array(vecid, value[], size = sizeof(value));
native vec_sort_unstable(vecid);
native vec_rsort_unstable(vecid);
native vec_split_at(vecid, &newLeftId, &newRightId);
native vec_rotate_left(vecid, mid);
native vec_rotate_right(vecid, k);
native vec_sort(vecid);
native vec_clone(vecid);
native vec_concat(vecid, id1, id2);
