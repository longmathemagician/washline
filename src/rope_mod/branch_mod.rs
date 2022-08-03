use std::sync::Arc;

use super::tree_mod::*;

#[derive(Debug)]
pub struct Branch {
	left_weight: usize,
	right_weight: usize,
	left: Option<Arc<Tree>>,
	right: Option<Arc<Tree>>,
}

/// The branch structure holds two options wrapping smart pointers to trees, along with the weight of each branch.
impl Branch {
	/// Returns a new branch containing the provided optioned links to tree elements.
	pub fn new(left: Option<Arc<Tree>>, right: Option<Arc<Tree>>) -> Self {
		let left_weight = if let Some(ref l) = left {
			l.get_weight()
		} else {
			0
		};
		let right_weight = if let Some(ref r) = right {
			r.get_weight()
		} else {
			0
		};
		Branch {
			left_weight,
			right_weight,
			left,
			right,
		}
	}

	/// Returns the weight of the branch.
	pub fn get_weight(&self) -> usize {
		self.left_weight + self.right_weight
	}

	/// Returns the weight of the left subtree in the branch. Used for parsing the rope in various methods.
	pub fn get_left_weight(&self) -> usize {
		self.left_weight
	}

	#[allow(dead_code)]
	/// Returns the weight of the right subtree in the branch. Not currently used.
	pub fn get_right_weight(&self) -> usize {
		self.right_weight
	}

	/// Returns a new option-wrapped link to the branch's left subtree.
	pub fn get_left(&self) -> Option<Arc<Tree>> {
		self.left.as_ref().map(Arc::clone)
	}

	/// Returns a new option-wrapped link to the branch's right subtree.
	pub fn get_right(&self) -> Option<Arc<Tree>> {
		self.right.as_ref().map(Arc::clone)
	}
}
