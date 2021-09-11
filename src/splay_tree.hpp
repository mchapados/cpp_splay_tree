#pragma once
#include <cstdint>
#include <functional>
#include <memory>
#include <iostream>
#include <vector>

template <typename T> class SplayTree {
	struct Node;
	using Child = std::unique_ptr<Node>;
	using ParentStack = std::vector<std::reference_wrapper<Child>>;
	// Each node in the tree
	struct Node {
		Child left;
		Child right;
		T data;
		// Create a node by moving left and right into the node, and copying in
		// the data.
		Node(const T &data)
		{
			this->left = nullptr;
			this->right = nullptr;
			this->data = data;
		}
	};

	// Tree root node
	Child root = nullptr;
	std::size_t size = 0;
	// Rewind stack for splaying
	ParentStack rewind;

	// Modifies tree below axis ONLY
	static inline void right_rotate(Child &axis)
	{
		// move x to temp
		Child temp = std::move(axis->left);
		// Make x our axis
		std::swap(axis, temp);
		// swap x's right child with y's left child
		std::swap(axis->right, temp->left);
		// swap y into x's right child (move old axis down)
		std::swap(axis->right, temp);
	}

	// Modifies tree below axis ONLY
	static inline void left_rotate(Child &axis)
	{
		// move x to temp
		Child temp = std::move(axis->right);
		// make x our axis
		std::swap(axis, temp);
		// swap x's left child with y's right child
		std::swap(axis->left, temp->right);
		// swap y into x's left child (move old axis down)
		std::swap(axis->left, temp);
	}

	static void splay(ParentStack &parents)
	{
		// Pull the value we are rotating to the root from the last visited node
		T &value = parents.back().get()->data;
		// Remove the value node from the stack, and move up to the axis
		parents.pop_back();
		// Splay up the stack until the value node is at the root.
		while (!parents.empty()) {
			Child &axis = parents.back();
			// the value we are rotating up is on the left of the axis
			// rotate to the right, to bring it to the root.
			if (axis->left != nullptr &&
			    value == axis->left->data) {
				right_rotate(axis);
				// the value we are rotating up is on the right of the axis
				// rotate it to the left, to bring it up to the root.
			} else {
				left_rotate(axis);
			}
			// Move the axis one step up in the tree
			parents.pop_back();
		}
	}

	static bool add(ParentStack &parents, const T &value, Child &n)
	{
		// Keep track of our trail as we go for splaying.
		parents.push_back(n);
		// If it's already here, then we won't add it.
		if (value == n->data) {
			return false;
			// Bigger, and can be added here. Add and return.
		} else if (value > n->data && n->right == nullptr) {
			n->right = std::make_unique<Node>(value);
			// Add the new node to the parents stack so that splay can find out
			// which node to rotate to the root.
			parents.push_back(n->right);
			return true;
			// Bigger, and n has children, call add on n's right child.
		} else if (value > n->data) {
			return add(parents, value, n->right);
			// Smaller, and can be added here. Add and return.
		} else if (value < n->data && n->left == nullptr) {
			n->left = std::make_unique<Node>(value);
			// Add the new node to the parents stack so that splay can find out
			// which node to rotate to the root.
			parents.push_back(n->left);
			return true;
			// Smaller, and n has children, call on n's left child.
		} else if (value < n->data) {
			return add(parents, value, n->left);
		}
		return false;
	}

	static bool contains(ParentStack &parents, Child &n, const T &value)
	{
		// Keep track of our trail as we go for splaying.
		parents.push_back(n);
		// Its bigger than n, so check the right
		if (value > n->data && n->right != nullptr)
			return contains(parents, n->right, value);
		// Its less than n, so check the left
		else if (value < n->data && n->left != nullptr)
			return contains(parents, n->left, value);
		// If it's not here, then it isn't in the tree.
		else if (value == n->data)
			return true;
		else
			return false;
	}

	static void in_order_print(const Node &n)
	{
		if (n.left != nullptr)
			in_order_print(*n.left);
		std::cout << "Contents: " << n.data << "\n";
		if (n.right != nullptr)
			in_order_print(*n.right);
	}

	static void structure_print(const Child &c, const std::uint32_t spacing)
	{
		if (c != nullptr) {
			structure_print(c->right, spacing + 5);
			std::cout << std::string(spacing, ' ') << c->data
				  << "\n";
			structure_print(c->left, spacing + 5);
		}
	}

    public:
	bool add(const T &value)
	{
		if (root == nullptr) {
			root = std::make_unique<Node>(value);
			++size;
			return true;
		} else {
			if (add(rewind, value, root)) {
				splay(rewind);
				++size;
				return true;
			}
			rewind.clear();
		}
		return false;
	}
	bool remove(const T &value)
	{
		// Tree is empty
		if (root == nullptr) {
			return false;
		}
		if (contains(value)) {
			// Decapitate the root
			Child left_tree = std::move(root->left);
			Child right_tree = std::move(root->right);
			// There is no left tree of the root, make the right subtree of root
			// into the main tree
			if (left_tree == nullptr) {
				// Old root is deallocated here
				root = std::move(right_tree);
			} else {
				// make the left subtree root; splay its maximum value; make the
				// right subtree root's right child
				// Old root is deallocated here
				root = std::move(left_tree);
				std::reference_wrapper<Child> curr = root;
				while (curr.get() != nullptr) {
					rewind.push_back(curr);
					curr = curr.get()->right;
				}
				splay(rewind);
				root->right = std::move(right_tree);
			}
			--size;
			return true;
		} else {
			return false;
		}
	}
	bool contains(const T &value)
	{
		// Root is null. It ain't here.
		if (root == nullptr) {
			return false;
		}
		// If we found it, rotate it to the root, and return true.
		bool found = contains(rewind, root, value);
		splay(rewind);
		return found;
	}
	void in_order_print(void)
	{
		if (root == nullptr) {
			std::cout << "Empty Splay Tree.\n";
		} else {
			in_order_print(*root);
		}
	}

	void structure_print(void)
	{
		structure_print(root, 0);
	}

	std::size_t get_size(void)
	{
		return size;
	}
};
