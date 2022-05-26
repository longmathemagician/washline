use super::tree_mod::*;
use std::sync::Arc;

pub struct ArcStack<T> { head: Vec<Arc<T>> }
impl<T> ArcStack<T> {
	pub fn new() -> Self { ArcStack {head: Vec::new()} }
	pub fn push(&mut self, next: Arc<T>) { self.head.push(next); }
	pub fn pop(&mut self) -> Option<Arc<T>>{ self.head.pop() }
	pub fn pop_two(&mut self) -> (Option<Arc<T>>, Option<Arc<T>>) {
		let last = self.head.pop();
		let second_last = self.head.pop();
		(last, second_last)
	}
	pub fn peek(&self) -> Option<Arc<T>> {
		let tmp = self.head.last();
		match tmp {
			Some(data) => Some(Arc::clone(data)),
			_ => None,
		}
	}
	#[allow(dead_code)]
	pub fn peek_two(&mut self) -> (Option<Arc<T>>, Option<Arc<T>>) {
		let left = self.head.len().checked_sub(2).map(|i| Arc::clone(&self.head[i]));
		let right = self.head.len().checked_sub(1).map(|i| Arc::clone(&self.head[i]));
		(left, right)
	}
}

pub struct TreeDFSStack {head: Vec<Arc<Tree>>, state: Vec<(bool, bool)>}
impl TreeDFSStack  {
	pub fn new() -> Self {TreeDFSStack {head: Vec::new(), state: Vec::new()}}
	pub fn push(&mut self, next: Arc<Tree>, state: (bool, bool)) {
		self.head.push(next);
		self.state.push(state);
	}
	pub fn pop(&mut self) -> Option<Arc<Tree>>{
		self.state.pop();
		self.head.pop()
	}
	pub fn peek_item(&self) -> Option<Arc<Tree>> {
		match self.head.last() {
			Some(arc) => Some(Arc::clone(arc)),
			_ => None,
		}

	}
	pub fn set_left_visited(&mut self, next_left_state: bool) {
		if let Some(current_state) = self.state.last() {
			let right = current_state.1;
			self.state.pop();
			self.state.push((next_left_state, right));
		}
	}
	pub fn set_right_visited(&mut self, next_right_state: bool) {
		if let Some(current_state) = self.state.last() {
			let left = current_state.0;
			self.state.pop();
			self.state.push((left, next_right_state));
		}
	}
	pub fn get_left_visited(&self) -> bool {
		let tmp = self.state.last();
		match tmp {
			Some(data) => (*data).0,
			_ => false,
		}
	}
	pub fn get_right_visited(&self) -> bool {
		let tmp = self.state.last();
		match tmp {
			Some(data) => (*data).1,
			_ => false,
		}
	}
}