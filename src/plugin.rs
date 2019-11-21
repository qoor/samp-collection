use samp::prelude::*;
use log::{info};

use crate::collection::*;

pub struct SampCollection {
	pub pawn_vecs: PawnAmxContainers,
	pub pawn_hashmaps: PawnAmxContainers
}

impl SampPlugin for SampCollection {
	fn on_load(&mut self) {
		info!("SA-MP Collection plugin was loaded.");
	}
	fn on_unload(&mut self) {
		info!("SA-MP Collection plugin was unloaded.");
	}

	fn on_amx_load(&mut self, amx: &Amx) {
		self.pawn_vecs.add_amx(&amx);
		self.pawn_hashmaps.add_amx(&amx);
	}
	fn on_amx_unload(&mut self, amx: &Amx) {
		self.pawn_vecs.remove_amx(&amx);
		self.pawn_hashmaps.remove_amx(&amx);
	}
}
