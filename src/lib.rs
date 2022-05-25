use std::mem;

pub struct Rope {
	head: Box<Tree>,
}

enum Tree {
	None,
	Branch(Branch),
	Leaf(Leaf),
}

struct Branch {
	left_weight: usize,
	right_weight: usize,
	left: Box<Tree>,
	right: Box<Tree>,
}

struct Leaf {
	length: usize,
	text: Box<str>,
}


struct TreeDFSStack<'a, T> {head: Vec<&'a T>, state: Vec<(bool, bool)>}
impl<'a, T> TreeDFSStack<'a, T>  {
	fn new() -> Self {TreeDFSStack {head: Vec::new(), state: Vec::new()}}
	fn push(&mut self, next: &'a T, state: (bool, bool)) {
		self.head.push(next);
		self.state.push(state);
	}
	fn pop(&mut self) -> Option<&'a T>{
		self.state.pop();
		self.head.pop()
	}
	fn peek_item(&self) -> Option<&'a T> {
		let tmp = self.head.last();
		match tmp {
			Some(data) => Some(*data),
			_ => None,
		}
	}
	fn set_left_visited(&mut self, next_left_state: bool) {
		if let Some(current_state) = self.state.last() {
			let right = current_state.1;
			self.state.pop();
			self.state.push((next_left_state, right));
		}
	}
	fn set_right_visited(&mut self, next_right_state: bool) {
		if let Some(current_state) = self.state.last() {
			let left = current_state.0;
			self.state.pop();
			self.state.push((left, next_right_state));
		}
	}
	fn get_left_visited(&self) -> bool {
		let tmp = self.state.last();
		match tmp {
			Some(data) => (*data).0,
			_ => false,
		}
	}
	fn get_right_visited(&self) -> bool {
		let tmp = self.state.last();
		match tmp {
			Some(data) => (*data).1,
			_ => false,
		}
	}
}

impl Leaf {
	fn new(data: String) -> Self {
		Leaf {
			length: data.len(),
			text: data.into_boxed_str(),
		}
	}

	fn get_weight(&self) -> usize {
		self.length
	}

	fn collect(&self) -> String {
		self.text.clone().to_string()
	}
}

impl Branch {
	fn new(left: Tree, right: Tree) -> Self {
		Branch {
			left_weight: left.get_weight(),
			right_weight: right.get_weight(),
			left: Box::new(left),
			right: Box::new(right),
		}
	}

	fn get_weight(&self) -> usize {
		self.left_weight + self.right_weight
	}

	fn get_left(&self) -> &Box<Tree> {
		&&(*&self.left)
	}

	fn get_right(&self) -> &Box<Tree> {
		&&(*&self.right)
	}
}

impl Tree {
	fn new_leaf(data: String) -> Tree {
		Tree::Leaf(Leaf::new(data))
	}

	fn new_branch(left: Tree, right: Tree) -> Tree {
		Tree::Branch(Branch::new(left, right))
	}

	fn new_boxed_branch(left: Tree, right: Tree) -> Box<Tree> {
		Box::new(Tree::Branch(Branch::new(left, right)))
	}

	fn get_weight(&self) -> usize {
		match self {
			Tree::None => 0,
			Tree::Leaf(leaf) => leaf.get_weight(),
			Tree::Branch(branch) => branch.get_weight(),
		}
	}
}

impl Rope {
	pub fn new() -> Self {
		Rope { head: Box::new(Tree::None) }
	}

	pub fn add_branch(&mut self, data: String) {
		let data_len: usize = data.len();
		let split_index = data_len - data_len / 2;

		let left_data = data[..split_index].to_string();
		let right_data = data[split_index..].to_string();

		let new_right_branch = Tree::new_branch(Tree::new_leaf(left_data), Tree::new_leaf(right_data));

		match *self.head {
			Tree::Branch(_) => self.head = Tree::new_boxed_branch(mem::replace(&mut self.head, Tree::None), new_right_branch),
			Tree::Leaf(_) => self.head = Tree::new_boxed_branch(mem::replace(&mut self.head, Tree::None), new_right_branch),
			Tree::None => self.head = Box::new(new_right_branch),
		}
	}

	pub fn collect_leaves (&self) -> String {
		let mut collected_text: String = String::new();
		let mut stack = TreeDFSStack::new();
		stack.push(&self.head, (false, false));

		while let Some(this_tree) = stack.peek_item() {
			if let Tree::Branch(branch) = &**this_tree {
				if !stack.get_left_visited() {
					stack.set_left_visited(true);
					stack.push(branch.get_left(), (false, false));
				}
				else if !stack.get_right_visited() {
					stack.set_right_visited(true);
					stack.push(branch.get_right(), (false, false));
				}
				else {
					stack.pop();
				}
			}
			else if let Tree::Leaf(leaf) = &**this_tree {
				collected_text.push_str(&leaf.collect());
				stack.pop();
			}
			else {stack.pop();}
		}

		collected_text
	}
}