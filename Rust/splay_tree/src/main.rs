mod splay_tree;

fn main() {
	let mut tree: splay_tree::SplayTree<u8> = splay_tree::SplayTree::new();
	tree.add(1);
	tree.add(6);
	tree.add(3);
	tree.add(4);
	tree.add(5);
	tree.add(2);
	println!(
		"1 is there: {}, 8 is there: {}",
		tree.contains(&1),
		tree.contains(&8)
	);
	tree.structure_print();
	println!("======");
	tree.remove(&3);
	tree.structure_print();
}
