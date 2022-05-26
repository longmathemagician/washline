use std::sync::Arc;

use super::tree_mod::*;

#[derive(Debug)]
pub struct Branch {
	left_weight: usize,
	right_weight: usize,
	left: Option<Arc<Tree>>,
	right: Option<Arc<Tree>>,
}
impl Branch {
	pub fn new(left: Option<Arc<Tree>>, right: Option<Arc<Tree>>) -> Self {
		let left_weight: usize;
		let right_weight: usize;
		match left {
			Some(ref l) => left_weight = l.get_weight(),
			_ => left_weight = 0,
		}
		match right {
			Some(ref r) => right_weight = r.get_weight(),
			_ => right_weight = 0,
		}
		Branch {
			left_weight,
			right_weight,
			left,
			right,
		}
	}
	pub fn get_weight(&self) -> usize {
		self.left_weight + self.right_weight
	}
	pub fn get_left_weight(&self) -> usize { self.left_weight }
	pub fn get_right_weight(&self) -> usize { self.right_weight }
	pub fn get_left(&self) -> Option<Arc<Tree>> {
		match &self.left {
			Some(data) => Some(Arc::clone(data)),
			None => None,
		}
	}
	pub fn get_right(&self) -> Option<Arc<Tree>> {
		match &self.right {
			Some(data) => Some(Arc::clone(data)),
			None => None,
		}
	}
}