#[derive(Debug)]
pub struct Leaf {
	length: usize,
	text: String,
}
impl Leaf {
	pub fn new(data: String) -> Self {
		Leaf {
			length: data.chars().count(),
			text: data,
		}
	}

	pub fn get_length(&self) -> usize {
		self.length
	}

	pub fn get_text(&self) -> String {
		self.text.to_string()
	}
}