use std::time::{Instant, Duration};

mod common;

use common::{TemporaryStorage, Context};

fn main() {
	let start_time = Instant::now();
	let context = Context{storage: TemporaryStorage::new()};
	let mut storage = context.storage;
	let bytes_to_store: Vec<u8> = vec![b'A', b'w', b',', b'y', b'e', b'a', b'h', b'!']; // Aw, yeah!
	let stored_bytes_length = bytes_to_store.len();
	let stored_bytes_address = storage.add_byte_vec(&bytes_to_store);
	let total_time = start_time.elapsed().as_secs_f64();
	print!("{} bytes stored at {} in temporary storage\n", stored_bytes_length, stored_bytes_address);

	let integers: Vec<i32> = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 0];
	let mut integers_as_bytes: Vec<u8> = Vec::new();
	for integer in integers 
	{
		for integer_as_byte in integer.to_be_bytes().iter() { integers_as_bytes.push(*integer_as_byte); }
	}

	let integers_as_bytes_length = integers_as_bytes.len();
	let integers_as_bytes_address = storage.add_byte_vec(&integers_as_bytes);
	print!("{} bytes stored at {} in temporary storage\n", integers_as_bytes_length, integers_as_bytes_address);
	print!("Total stored in temporary storage: {}\n", storage.occupied);

	storage.reset_storage();
	print!("Total stored in storage after reset: {}\n", storage.occupied);

	print!("Program completed in {} seconds or {} milliseconds.\n", total_time, total_time * 1000.0);
}
