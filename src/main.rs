use std::time::{Instant, Duration};

mod common;

use common::{TemporaryStorage, Context, Logger};

use std::env::current_dir as current_directory;
use std::env::args as command_line_arguments;
use std::fs as file_system;
use std::path::{Path, PathBuf};

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

fn logger_and_memory_example()
{
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

fn help_text()
{
	print!(
"elega_core\n"
	);
}

fn initialize_project(project_path: &String)
{
	print!("Creating project at {}...\n", project_path);
	let project_directory_create_init = file_system::create_dir(project_path).unwrap();

	let current_working_directory: String = current_directory().unwrap().display().to_string();
	let mut project_template_path: String = current_working_directory.clone();
	project_template_path.push_str("\\_project_template\\src\\");
	print!("Copying template files from {}...\n", project_template_path);

	for file_name_result in file_system::read_dir(project_template_path).unwrap()
	{
		let file_name_dir_entry = file_name_result.unwrap();
		let path_as_string = file_name_dir_entry.path().display().to_string();
		print!("Copying {}...\n", path_as_string);

		let mut file_copied: String = path_as_string.clone();
		let file_copied_without_cwd = &format_args!("{}", current_working_directory).to_string();
		file_copied = file_copied.replace(file_copied_without_cwd, "");

		let mut copy_to_path = project_path.clone();
		copy_to_path.push('\\');
		copy_to_path.push_str(&file_copied);

		print!("copy_to_path: {}\n", copy_to_path);

		file_system::copy(path_as_string, copy_to_path);
	}
}

fn main() 
{
	let mut previous_argument = String::new();
	for argument in command_line_arguments()
	{
		if argument == "--help" || argument == "-h"
		{ help_text(); return; }

		if previous_argument == "--init-project" || previous_argument == "-ip"
		{
			initialize_project(&argument);
			return;
		}

		if argument == "--example"
		{ logger_and_memory_example(); return; }
		
		previous_argument = argument;
	}
}
