use std::sync::Arc;

use super::branch_mod::*;
use super::leaf_mod::*;

#[derive(Debug)]
pub enum Tree {
	Branch(Branch),
	Leaf(Leaf),
}
impl Tree {
	pub fn new_leaf(data: String) -> Tree {
		Tree::Leaf(Leaf::new(data))
	}
	pub fn new_branch(left: Option<Arc<Tree>>, right: Option<Arc<Tree>>) -> Option<Arc<Tree>> {
		Some(Arc::new(Tree::Branch(Branch::new(left, right))))
	}
	pub fn get_weight(&self) -> usize {
		match self {
			Tree::Branch(branch) => branch.get_weight(),
			Tree::Leaf(leaf) => leaf.get_length(),
		}
	}
}
