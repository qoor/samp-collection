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

impl PawnValue {
	pub fn get_int_value(&self) -> Result<i32, ()> {
		match self {
			PawnValue::Integer(value) => Ok(*value),
			_ => Err(())
		}
	}
	pub fn get_float_value(&self) -> Result<f32, ()> {
		match self {
			PawnValue::Float(value) => Ok(*value),
			_ => Err(())
		}
	}
	pub fn get_array_value(&self, length: i32) -> Result<&[i32], ()> {
		match &self {
			PawnValue::Array(value) => {
				let array: &PawnArray = value;

				Ok(&array.0[0..(length as usize)])
			},
			_ => Err(())
		}
	}
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
