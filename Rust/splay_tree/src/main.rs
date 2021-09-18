mod splay_tree;
use std::fs::File;
use std::io::Read;
use std::path::Path;
use std::sync::{
	atomic::{AtomicBool, AtomicU8, Ordering},
	Arc, Mutex,
};
use std::thread;

fn french_tree() {
	let mut arbre: splay_tree::SplayTree<&str> = splay_tree::SplayTree::new();
	arbre.add("const mot_simple<char> &valeur");
	arbre.add("banane");
	arbre.add("fraise");
	arbre.add("kiwi");
	arbre.add("sirop d'érable");
	arbre.add("crêpe");
	arbre.structure_print();
}

fn main() {
	let tree: Arc<Mutex<splay_tree::SplayTree<u8>>> =
		Arc::new(Mutex::new(splay_tree::SplayTree::new()));
	let shutdown: Arc<AtomicBool> = Arc::new(AtomicBool::new(false));
	let some_byte: Arc<AtomicU8> = Arc::new(AtomicU8::new(0));
	let (tr, sd, sb) = (
		Arc::clone(&tree),
		Arc::clone(&shutdown),
		Arc::clone(&some_byte),
	);
	let t1 = thread::spawn(move || {
		// Open up urandom
		let path = Path::new("/dev/urandom");
		let urandom = match File::open(&path) {
			Err(w) => panic!("Error. Unable to open urandom: {}", w),
			Ok(urandom) => urandom,
		};
		// Keep adding bytes to the splay tree until we are told to shutdown.
		while !sd.load(Ordering::SeqCst) {
			let mut read_byte: [u8; 1] = Default::default();
			match (&mut (&urandom)).read_exact(&mut read_byte) {
				Err(w) => panic!("Error reading from urandom: {}", w),
				Ok(_) => (),
			}
			sb.store(read_byte[0], Ordering::SeqCst);
			{
				let mut tree = tr.lock().unwrap();
				tree.add(read_byte[0]);
			}
		}
		0
	});
	loop {
		let has: bool;
		{
			let mut tree = tree.lock().unwrap();
			has = tree.contains(&42);
		}
		if has {
			shutdown.store(true, Ordering::SeqCst);
			break;
		} else {
			{
				let mut tree = tree.lock().unwrap();
				tree.remove(&some_byte.load(Ordering::SeqCst));
			}
		}
	}
	t1.join().unwrap();
	{
		let tree = tree.lock().unwrap();
		tree.structure_print();
		println!(
			"Ending tree size (maximum tree size is 256): {}",
			tree.size()
		);
	}
	french_tree();
}
