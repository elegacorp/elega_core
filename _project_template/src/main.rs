
mod common;
use common::{Logger, Context};

fn initialize_context_and_logger() -> Context
{
	let mut context_logger: Logger = Logger::new(); // Default logger configuration
	context_logger.print_all_on = true; // Prints all debug levels
	context_logger.print_asap = true; // Prints immediately instead of on publish() call, or some other time

	let mut logging_directory = current_directory().unwrap().display().to_string();
	logging_directory.push_str("\\log.txt"); // Default directory is current working directory + log.txt
	context_logger.file_path = logging_directory;

	let context = Context{storage: TemporaryStorage::new(), logger: context_logger };
	return context;
}

fn main()
{
	let mut context: &mut Context = initialize_context_and_logger();
	context.logger.log_info("Hello world!");
}