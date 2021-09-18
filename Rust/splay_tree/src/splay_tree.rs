use std::{fmt::Display, mem};

type Child<T> = Option<Box<Node<T>>>;

struct Node<T: Ord + Display> {
	data: T,
	left: Child<T>,
	right: Child<T>,
}

impl<T: Ord + Display> Node<T> {
	fn new(data: T) -> Node<T> {
		Node {
			data: data,
			left: None,
			right: None,
		}
	}
}

pub struct SplayTree<T: Ord + Display> {
	root: Child<T>,
	size: usize,
	rewind: Vec<*mut Child<T>>,
}

unsafe impl<T: Ord + Display> Send for SplayTree<T> {}

impl<T: Ord + Display> SplayTree<T> {
	pub fn new() -> SplayTree<T> {
		SplayTree {
			root: None,
			rewind: Default::default(),
			size: 0,
		}
	}

	fn right_rotate(axis: &mut Child<T>) {
		// move x to temp
		let mut temp = match axis {
			Some(axis) => axis.left.take(),
			None => panic!("axis is None in right rotate."),
		};
		// Make x our axis
		mem::swap(axis, &mut temp);
		// swap x's right child with y's left child
		let (axis, temp_un): (&mut Box<Node<T>>, &mut Box<Node<T>>) =
			match (axis.as_mut(), temp.as_mut()) {
				(Some(axis), Some(temp)) => (axis, temp),
				_ => panic!("axis and temp is null for child swap."),
			};
		mem::swap(&mut axis.right, &mut temp_un.left);
		// swap y into x's right child (move old axis down)
		mem::swap(&mut axis.right, &mut temp);
	}

	fn left_rotate(axis: &mut Child<T>) {
		// move x to temp
		let mut temp = match axis.as_mut() {
			Some(axis) => axis.right.take(),
			None => panic!("axis is None in right rotate."),
		};
		// make x our axis
		mem::swap(axis, &mut temp);
		// swap x's left child with y's right child
		let (axis, temp_un): (&mut Box<Node<T>>, &mut Box<Node<T>>) =
			match (axis.as_mut(), temp.as_mut()) {
				(Some(axis), Some(temp)) => (axis, temp),
				_ => panic!("axis and temp is null for child swap."),
			};
		mem::swap(&mut axis.left, &mut temp_un.right);
		// swap y into x's left child (move old axis down)
		mem::swap(&mut axis.left, &mut temp);
	}

	fn splay(rewind: &mut Vec<*mut Child<T>>) {
		let value: &T = unsafe { &(*rewind.pop().unwrap()).as_ref().unwrap().data };
		while rewind.len() > 0 {
			let axis: &mut Child<T> = unsafe { &mut *(rewind.pop().unwrap()) };
			match &axis.as_ref().unwrap().left {
				Some(left) if value == &left.data => {
					SplayTree::right_rotate(axis);
				}
				_ => SplayTree::left_rotate(axis),
			}
		}
	}

	pub fn contains(&mut self, value: &T) -> bool {
		match self.root {
			None => return false,
			_ => (),
		}
		let mut found = false;
		let mut current = &mut self.root;
		while !current.is_none() {
			self.rewind.push(current);
			let current_unwrap = match current.as_mut() {
				Some(current) => current,
				None => return false,
			};
			if value > &current_unwrap.data {
				current = &mut current_unwrap.right;
			} else if value < &current_unwrap.data {
				current = &mut current_unwrap.left;
			} else {
				found = true;
				break;
			}
		}
		SplayTree::splay(&mut self.rewind);
		found
	}

	pub fn remove(&mut self, value: &T) -> bool {
		match self.root {
			None => return false,
			_ => (),
		}
		if self.contains(value) {
			let to_delete = self.root.as_mut().unwrap();
			let left_tree = to_delete.left.take();
			let right_tree = to_delete.right.take();
			match left_tree {
				None => {
					self.root = right_tree;
				}
				Some(_) => {
					self.root = left_tree;
					let mut current = &mut self.root;
					while !current.is_none() {
						self.rewind.push(current);
						current = &mut current.as_mut().unwrap().right;
					}
					SplayTree::splay(&mut self.rewind);
					self.root.as_mut().unwrap().right = right_tree;
				}
			}
			self.size -= 1;
			true
		} else {
			false
		}
	}

	pub fn add(&mut self, value: T) -> bool {
		match self.root {
			None => {
				self.root = Some(Box::new(Node::new(value)));
				self.size += 1;
				true
			}
			Some(_) => {
				if !self.contains(&value) {
					let mut new_node = Node::new(value);
					let root = self.root.as_mut().unwrap();
					if new_node.data > root.data {
						new_node.right = root.right.take();
						new_node.left = self.root.take();
					} else {
						new_node.left = root.left.take();
						new_node.right = self.root.take();
					}
					self.root = Some(Box::new(new_node));
					self.size += 1;
					true
				} else {
					false
				}
			}
		}
	}

	fn structure_print_priv(tree: &Option<Box<Node<T>>>, spacing: u32) {
		match tree {
			Some(t) => {
				SplayTree::structure_print_priv(&t.right, spacing + 5);
				let mut node_rep = String::with_capacity(spacing as usize);
				if spacing > 0 {
					for _ in 0..spacing - 1 {
						node_rep.push(' ');
					}
				}
				println!("{}{}", &node_rep, &t.data);
				SplayTree::structure_print_priv(&t.left, spacing + 5);
			}
			_ => (),
		}
	}

	pub fn structure_print(&self) {
		SplayTree::structure_print_priv(&self.root, 0);
	}

	pub fn size(&self) -> usize {
		self.size
	}
}
