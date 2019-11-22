use std::collections::*;

use samp::prelude::*;
use log::{info};

use crate::collection::*;
use crate::pawniter::PawnAmxIters;
use crate::value::PawnValue;

pub struct SampCollection<'a> {
	pub pawn_vecs: PawnAmxContainers<Vec<PawnValue>>,
	pub pawn_vec_iters: PawnAmxIters<'a, Vec<PawnValue>>,

	pub pawn_hashmaps: PawnAmxContainers<HashMap<PawnValue, PawnValue>>
}

impl<'a> SampPlugin for SampCollection<'a> {
	fn on_load(&mut self) {
		info!("SA-MP Collection plugin was loaded.");
	}
	fn on_unload(&mut self) {
		info!("SA-MP Collection plugin was unloaded.");
	}

	fn on_amx_load(&mut self, amx: &Amx) {
		self.pawn_vecs.add_amx(amx);
		self.pawn_vec_iters.add_amx(amx);

		self.pawn_hashmaps.add_amx(amx);
	}
	fn on_amx_unload(&mut self, amx: &Amx) {
		self.pawn_vecs.remove_amx(amx);
		self.pawn_vec_iters.remove_amx(amx);

		self.pawn_hashmaps.remove_amx(amx);
	}
}
