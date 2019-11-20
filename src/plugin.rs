use samp::prelude::*;
use log::{info};

pub struct SampCollection;

impl SampPlugin for SampCollection {
	fn on_load(&mut self) {
		info!("SA-MP Collection plugin was loaded.");
	}
	fn on_unload(&mut self) {
		info!("SA-MP Collection plugin was unloaded.");
	}

	fn on_amx_load(&mut self, amx: &Amx) {

	}
	fn on_amx_unload(&mut self, amx: &Amx) {
		
	}
}
