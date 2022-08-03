use std::sync::Arc;

use super::branch_mod::*;
use super::leaf_mod::*;

#[derive(Debug)]
pub enum Tree {
	Branch(Branch),
	Leaf(Leaf),
}

/// The tree enum is the base data type of the rope, and holds either a leaf element or a branch.
impl Tree {
	/// Create a new leaf from the provided text, and return it wrapped in a tree.
	pub fn new_leaf(data: String) -> Tree {
		Tree::Leaf(Leaf::new(data))
	}

	/// Create a new branch from the provided tree links, and return it wrapped in a tree wrapped in an option.
	pub fn new_branch(left: Option<Arc<Tree>>, right: Option<Arc<Tree>>) -> Option<Arc<Tree>> {
		Some(Arc::new(Tree::Branch(Branch::new(left, right))))
	}

	/// Returns the weight of the tree.
	pub fn get_weight(&self) -> usize {
		match self {
			Tree::Branch(branch) => branch.get_weight(),
			Tree::Leaf(leaf) => leaf.get_length(),
		}
	}
}
