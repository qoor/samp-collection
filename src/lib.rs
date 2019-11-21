mod plugin;
mod value;
mod idallocator;
mod collection;
mod natives;

use samp::{initialize_plugin};

use plugin::SampCollection;
use collection::PawnAmxContainers;

initialize_plugin!(
    natives: [
        SampCollection::vec_new,
        SampCollection::vec_with_capacity,
        SampCollection::vec_drop,
        SampCollection::vec_capacity
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
