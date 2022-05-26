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
		let left_weight = if let Some(ref l) = left { l.get_weight() } else { 0 };
		let right_weight = if let Some(ref r) = right { r.get_weight() } else { 0 };
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
	#[allow(dead_code)]
	pub fn get_right_weight(&self) -> usize { self.right_weight }
	pub fn get_left(&self) -> Option<Arc<Tree>> {
		self.left.as_ref().map(Arc::clone)
	}
	pub fn get_right(&self) -> Option<Arc<Tree>> {
		self.right.as_ref().map(Arc::clone)
	}
}