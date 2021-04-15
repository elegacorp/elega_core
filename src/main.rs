use std::time::{Instant, Duration};

#[macro_use]
mod common;

use common::{TemporaryStorage, Context, Logger};

use std::env::current_dir as current_directory;

fn log_storage_amount(stored_bytes_length: usize, stored_bytes_address: usize) -> String
{
	// Log byte amount stored at specified index (or address of temporary storage)
	let mut log_message: String = stored_bytes_length.to_string();
	log_message.push_str(&String::from(" bytes stored at "));
	log_message.push_str(&stored_bytes_address.to_string());
	log_message.push_str(&String::from(" in temporary storage\n"));
	return log_message;
}

fn log_total_stored_in_storage(storage_occupied: i32) -> String
{
	let mut log_message: String = String::from("Total stored in temporary storage: ");
	log_message.push_str(&storage_occupied.to_string());
	log_message.push('\n');
	return log_message;
}

fn total_run_time_message(total_time: f64) -> String
{
	let mut log_message: String = String::from("Program completed in ");
	log_message.push_str(&total_time.to_string());
	log_message.push_str(&String::from(" seconds or "));
	log_message.push_str(&(total_time * 1000.0).to_string());
	log_message.push_str(" milliseconds\n");
	return log_message;
}

fn main() {
	let start_time = Instant::now();

	let mut context_logger: Logger = Logger::new();
	context_logger.print_all_on = true;
	context_logger.print_asap = true;

	let mut logging_directory = current_directory().unwrap().display().to_string();
	logging_directory.push_str("\\log.txt");
	context_logger.file_path = logging_directory;

	let mut context = Context{storage: TemporaryStorage::new(), logger: context_logger };
	let mut storage = context.storage;
	let bytes_to_store: Vec<u8> = vec![b'A', b'w', b',', b'y', b'e', b'a', b'h', b'!']; // Aw, yeah!
	let stored_bytes_length = bytes_to_store.len();
	let stored_bytes_address = storage.add_byte_vec(&bytes_to_store);
	
	context.logger.log_info(&log_storage_amount(stored_bytes_length, stored_bytes_address));
	
	// Alternative way to construct messages that can format appropriately similar to
	// what print! would do under the hood...
	// let message = format_args!("{} bytes stored at address {} \n", stored_bytes_length, stored_bytes_address).to_string();
	// context.logger.log_info(&message);

	let integers: Vec<i32> = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 0];
	let mut integers_as_bytes: Vec<u8> = Vec::new();
	for integer in integers 
	{
		for integer_as_byte in integer.to_be_bytes().iter() { integers_as_bytes.push(*integer_as_byte); }
	}

	let integers_as_bytes_length = integers_as_bytes.len();
	let integers_as_bytes_address = storage.add_byte_vec(&integers_as_bytes);
	
	log_storage_amount(integers_as_bytes_length, integers_as_bytes_address);

	context.logger.log_info(&log_total_stored_in_storage(storage.occupied));

	storage.reset_storage();
	context.logger.log_info(&log_total_stored_in_storage(storage.occupied));

	let total_time: f64 = start_time.elapsed().as_secs_f64();
	context.logger.log_info(&total_run_time_message(total_time));

	context.logger.publish();
}
