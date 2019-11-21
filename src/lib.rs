mod plugin;
mod value;
mod idallocator;
mod collection;
mod natives;
mod pawniter;

use samp::{initialize_plugin};

use plugin::SampCollection;
use collection::PawnAmxContainers;

initialize_plugin!(
    natives: [
        SampCollection::vec_new,
        SampCollection::vec_with_capacity,
        SampCollection::vec_drop,
        SampCollection::vec_capacity,
        SampCollection::vec_reserve,
        SampCollection::vec_reserve_exact,
        SampCollection::vec_shrink_to_fit,
        SampCollection::vec_truncate,
        SampCollection::vec_unsafe_set_len,
        SampCollection::vec_swap_remove,
        SampCollection::vec_insert_int,
        SampCollection::vec_insert_float,
        SampCollection::vec_insert_array,
        SampCollection::vec_remove_int,
        SampCollection::vec_remove_float,
        SampCollection::vec_remove_array,
        SampCollection::vec_push_int,
        SampCollection::vec_push_float,
        SampCollection::vec_push_array,
        SampCollection::vec_pop_int,
        SampCollection::vec_pop_float,
        SampCollection::vec_pop_array,
        SampCollection::vec_get_type,
        //SampCollection::vec_append
        SampCollection::vec_drain,
        SampCollection::vec_clear,
        SampCollection::vec_len,
        SampCollection::vec_split_off,
        SampCollection::vec_resize_int,
        SampCollection::vec_resize_float,
        SampCollection::vec_resize_array,
        SampCollection::vec_dedup,
        SampCollection::vec_is_empty,
        SampCollection::vec_get_int,
        SampCollection::vec_get_float,
        SampCollection::vec_get_array,
        SampCollection::vec_first_int,
        SampCollection::vec_first_float,
        SampCollection::vec_first_array,
        SampCollection::vec_last_int,
        SampCollection::vec_last_float,
        SampCollection::vec_last_array
    ],
    {
        let samp_logger = samp::plugin::logger()
            .level(log::LevelFilter::Info);
        
        let _ = fern::Dispatch::new()
            .format(|callback, message, record| {
                callback.finish(format_args!("[collection] [{}]: {}", record.level().to_string().to_lowercase(), message))
            })
            .chain(samp_logger)
            .apply();
        
        plugin::SampCollection {
            pawn_vecs: PawnAmxContainers::new(),
            pawn_hashmaps: PawnAmxContainers::new()
        }
    }
);
