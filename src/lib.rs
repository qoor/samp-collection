use samp::prelude::*;
use samp::{initialize_plugin};

mod plugin;
mod value;
mod idallocator;
mod collections;

initialize_plugin!(
    natives: [

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

        }
    }
);
