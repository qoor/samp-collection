pub struct PawnArray {
	data: Vec<i32>,
	length: i32
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

				Ok(&array.data[0..(length as usize)])
			},
			_ => Err(())
		}
	}
}
