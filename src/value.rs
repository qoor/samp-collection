pub struct PawnArray(Vec<i32>);

impl PawnArray {
	pub fn new(data: Vec<i32>) -> PawnArray {
		Self(data)
	}
	pub fn len(&self) -> usize {
		self.0.len()
	}
	pub fn as_ptr(&self) -> *const i32 {
		self.0.as_ptr()
	}
}

pub enum PawnExportValueType {
	Integer = 1,
	Float = 2,
	Array = 3
}

pub enum PawnValue {
	Integer(i32),
	Float(f32),
	Array(PawnArray)
}

impl Clone for PawnValue {
	fn clone(&self) -> PawnValue {
		match self {
			PawnValue::Integer(value) => PawnValue::Integer(*value),
			PawnValue::Float(value) => PawnValue::Float(*value),
			PawnValue::Array(value) => {
				let data = value.0.clone();

				PawnValue::Array(PawnArray::new(data))
			}
		}
	}
}

impl PartialEq for PawnValue {
	fn eq(&self, other: &PawnValue) -> bool{
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
				return self_value.0 == other_value.0;
			}
		}

		false
	}
}
