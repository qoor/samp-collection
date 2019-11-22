use std::cmp::Ordering;

pub enum PawnExportValueType {
	Integer = 1,
	Float = 2,
	Array = 3
}

pub enum PawnValue {
	Integer(i32),
	Float(f32),
	Array(Vec<i32>)
}

impl Clone for PawnValue {
	fn clone(&self) -> PawnValue {
		match self {
			PawnValue::Integer(value) => PawnValue::Integer(*value),
			PawnValue::Float(value) => PawnValue::Float(*value),
			PawnValue::Array(value) => {
				let data = value.clone();

				PawnValue::Array(data)
			}
		}
	}
}

impl PartialEq for PawnValue {
	fn eq(&self, other: &PawnValue) -> bool {
		if let PawnValue::Integer(self_value) = self {
			if let PawnValue::Integer(other_value) = other {
				return self_value == other_value;
			}
		} else if let PawnValue::Float(self_value) = self {
			if let PawnValue::Float(other_value) = other {
				return self_value == other_value;
			}
		} else if let PawnValue::Array(self_value) = self {
			if let PawnValue::Array(other_value) = other {
				return self_value == other_value;
			}
		}

		false
	}
}

impl Ord for PawnValue {
	fn cmp(&self, other: &PawnValue) -> Ordering {
		match self {
			PawnValue::Integer(self_value) => {
				match other {
					PawnValue::Integer(other_value) => {
						if self_value > other_value {
							Ordering::Greater
						} else if self_value < other_value {
							Ordering::Less
						} else {
							Ordering::Equal
						}
					},
					PawnValue::Float(_) => {
						Ordering::Less
					},
					PawnValue::Array(_) => {
						Ordering::Less
					}
				}
			},
			PawnValue::Float(self_value) => {
				match other {
					PawnValue::Float(other_value) => {
						if self_value > other_value {
							Ordering::Greater
						} else if self_value < other_value {
							Ordering::Less
						} else {
							Ordering::Equal
						}
					},
					PawnValue::Integer(_) => {
						Ordering::Greater
					},
					PawnValue::Array(_) => {
						Ordering::Less
					}
				}
			},
			PawnValue::Array(self_value) => {
				match other {
					PawnValue::Array(other_value) => {
						if self_value > other_value {
							Ordering::Greater
						} else if self_value < other_value {
							Ordering::Less
						} else {
							Ordering::Equal
						}
					},
					PawnValue::Integer(_) => {
						Ordering::Greater
					},
					PawnValue::Float(_) => {
						Ordering::Greater
					}
				}
			}
		}
	}
}

impl PartialOrd for PawnValue {
	fn partial_cmp(&self, other: &PawnValue) -> Option<Ordering> {
		Some(self.cmp(other))
	}
}

impl Eq for PawnValue {
	
}
