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
	pub fn cat(&mut self, data: String) {
		let string_len: usize = data.chars().count();
		let split_index = string_len / 2;
		let left_data = data.chars().take(split_index).collect();
		let right_data = data.chars().take(string_len).skip(split_index).collect();
		
		let new_right_branch = Tree::new_branch(Some(Arc::new(Tree::new_leaf(left_data))), Some(Arc::new(Tree::new_leaf(right_data))));

		if self.head.is_some() {
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
		while leaf_stack.peek().is_some() {
			let (last, second_last) = leaf_stack.pop_two();

			let new_branch = Tree::new_branch(last, second_last);
			if new_head.is_some() {
				new_head = Tree::new_branch(new_head.take(), new_branch);
			}
			else {
				new_head = new_branch;
			}
		}
		self.head = new_head.take();
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
				}
				else if let Some(left_sub_branch) = branch.get_left() {
						tree_stack.push(left_sub_branch, (false, false));
				} else { return None }
			}
			else if let Tree::Leaf(leaf) = &*current_tree {
				return Some(leaf.get_text().chars().nth(char_index).unwrap());
			}
		}
		None
	}
	/// Substring returns an optioned String representing the desired substring
	/// of the document. 
	pub fn substring(&self, mut i: usize, j: usize) -> Option<String> {
		// this will accumulate our output
		let mut output_string = String::new();

		// if an invalid range was requested, abort mission
		if i>j { return None }

		// accumulates left weights as we traverse the tree
		let mut accum_left: usize = 0;

		let mut stack: Vec<Arc<Tree>> = Vec::new(); // stack of tree elements
		let mut stack_offsets: Vec<usize> = Vec::new() // stack to record position offsets
		let rope_length: usize;
		match &self.head {
			Some(tree) => {
				stack.push(Arc::clone(tree)); // push the root node to the stack
				stack_offsets.push(0);
				rope_length = tree.get_weight(); // and record the length
			},
			_ => return None, // if this rope is None, it has no substrings
		}

		if j>rope_length { return None } // return None if the range is invalid

		// loop through tree
		while i < j { if let Some(tree) = stack.last() {
			if let Tree::Branch(branch) = &**tree { // we're in a branch
				// either go left, right, or up
				if (i+accum_left) < branch.get_left_weight() { // push into left
					if let Some(nxt_bra) = branch.get_left() {
						stack.push(nxt_bra);
					} else { return None }
				}
				else if (i+accum_left) < branch.get_weight() { // push into right
					if let Some(nxt_bra) = branch.get_left() {
						accum_left += branch.get_left_weight();
						stack.push(nxt_bra);
						stack_offsets.push(branch.get_left_weight());
						todo!();
					} else { return None }
				}
				else {
					stack.pop();
				}
			}
			else if let Tree::Leaf(leaf) = &**tree { // we're in a leaf
				let leaf_length = leaf.get_length();
				if j-i > leaf_length {
					// add all of this leaf to the output and go up for more
					let tmp: String = leaf.get_text().chars().take(leaf_length).skip(i).collect();
					i = 0; // pull from the beginning of the next leaf
					output_string.push_str(&tmp);
					stack.pop(); // go back up for more
				}
				else {
					// this leaf contains all of the text needed
					let tmp: String = leaf.get_text().chars().take(j).skip(i).collect();
					output_string.push_str(&tmp);
					return Some(output_string);
				}
			}
		} else { break }} // exit the loop if we empty the stack 

		// should never exit from here unless the tree is broken, so return None
		None
	}
}
