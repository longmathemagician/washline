use std::mem;
use std::sync::Arc;

mod stack_mod;
mod tree_mod;
mod leaf_mod;
mod branch_mod;

use stack_mod::*;
use tree_mod::*;

#[derive(Debug)]
pub struct Rope {
	head: Option<Arc<Tree>>,
}
impl Rope {
	pub fn new() -> Self {
		Rope { head: None }
	}
	fn split_string(string: String) -> (String, String) {
		let string_len: usize = string.chars().count();
		let split_index = string_len / 2;
		let left = string.chars().take(split_index).collect();
		let right = string.chars().take(string_len).skip(split_index).collect();
		(left, right)
	}
	pub fn add_branch(&mut self, data: String) {
		let (left_data, right_data) = Rope::split_string(data);
		let new_right_branch = Tree::new_branch(Some(Arc::new(Tree::new_leaf(left_data))), Some(Arc::new(Tree::new_leaf(right_data))));

		if let Some(_) = self.head {
			self.head = Tree::new_branch(mem::replace(&mut self.head, None), new_right_branch);
		}
		else {
			self.head = new_right_branch;
		}
	}
	pub fn get_text(&self) -> String {
		let mut collected_text: String = String::new();
		let mut stack: ArcStack<Tree> = self.collect_leaves();
		while let Some(current_tree) = stack.peek() {
			if let Tree::Leaf(leaf) = &*current_tree {
				collected_text += &leaf.get_text();
			}
			stack.pop();
		}
		collected_text
	}
	fn collect_leaves(&self) -> ArcStack<Tree> {
		let mut collected_leaves: ArcStack<Tree> = ArcStack::new();
		let mut tree_stack: TreeDFSStack = TreeDFSStack::new();

		if let Some(tree) = &self.head {
			tree_stack.push(Arc::clone(tree), (false, false));
		}

		while let Some(this_tree) = tree_stack.peek_item() {
			if let Tree::Branch(branch) = &*this_tree {
				if !tree_stack.get_left_visited() {
					tree_stack.set_left_visited(true);
					if let Some(left_sub_branch) = branch.get_left() {
						tree_stack.push(left_sub_branch, (false, false));
					}
				}
				else if !tree_stack.get_right_visited() {
					tree_stack.set_right_visited(true);
					if let Some(right_sub_branch) = branch.get_right() {
						tree_stack.push(right_sub_branch, (false, false));
					}
				}
				else {
					tree_stack.pop();
				}
			}
			else if let Tree::Leaf(leaf) = &*this_tree {
				if leaf.get_length() > 0 { // drop empty leaves
					collected_leaves.push(Arc::clone(&this_tree));
				}
				tree_stack.pop();
			}
			else {
				tree_stack.pop();
			}
		}

		collected_leaves.reverse();
		collected_leaves
	}
	pub fn rebuild(&mut self) {
		let mut leaf_stack: ArcStack<Tree> = self.collect_leaves();
		let mut new_head: Option<Arc<Tree>> = None;
		while let Some(_) = leaf_stack.peek() {
			let (last, second_last) = leaf_stack.pop_two();

			let new_branch = Tree::new_branch(last, second_last);
			if let Some(_) = new_head {
				new_head = Tree::new_branch(mem::replace(&mut new_head, None), new_branch);
			}
			else {
				new_head = new_branch;
			}
		}
		self.head = mem::replace(&mut new_head, None);
	}
	pub fn char_at(&self, source_index: usize) -> Option<char> {
		let mut tree_stack: TreeDFSStack = TreeDFSStack::new();
		if let Some(tree) = &self.head {
			tree_stack.push(Arc::clone(tree), (false, false));
		}

		let mut char_index: usize;
		let doc_length: usize;

		if let Some(head) = &self.head {
			doc_length = head.get_weight() - 1;
			if source_index > doc_length { return None }
			char_index = source_index;
		} else { return None }

		while let Some(current_tree) = tree_stack.peek_item() {
			if let Tree::Branch(branch) = &*current_tree {
				if char_index >= branch.get_left_weight() {
					if let Some(right_sub_branch) = branch.get_right() {
						char_index -= branch.get_left_weight();
						tree_stack.push(right_sub_branch, (false, false));
					} else { return None }
				} else {
					if let Some(left_sub_branch) = branch.get_left() {
						tree_stack.push(left_sub_branch, (false, false));
					} else { return None }
				}
			}
			else if let Tree::Leaf(leaf) = &*current_tree {
				return Some(leaf.get_text().chars().nth(char_index).unwrap());
			}
		}
		None
	}
}
