// Context and TemporaryStorage are based on ideas that come standard with Thekla Inc's 
// Jai programming language. See Jonathan Blow's session on discussing how Jai implemented
// the overall context and temporary storage concepts as a part of its normal language features
// in this video: 
// https://www.youtube.com/watch?v=SSVHWrYG974
// 
// The below code does not perfectly replicate what is done by Blow in the above link, and is 
// not meant to precisely. Instead, these are meant to serve as useful structures as they apply
// to projects using this code written in Rust. The below code likely does not make use of 
// pointers or dynamic allocation in the same way as Blow does so in the Jai language.
// 
// 																		-Scott L.

pub struct Context
{
	pub storage: TemporaryStorage,
}

const DEFAULT_STORAGE_SIZE: usize = 40960;
const DEFAULT_ALLOCATOR_SIZE: usize = 7168;
const NULL: u8 = b'\0';

pub struct Allocator
{
	pub index: usize,
	pub data: [u8; DEFAULT_ALLOCATOR_SIZE],
}

pub struct TemporaryStorage
{
	pub data: [u8; DEFAULT_STORAGE_SIZE],
	pub occupied: i32,
	pub max_usage: i32,
	pub allocator: Allocator,
}

impl TemporaryStorage
{
	pub fn new() -> TemporaryStorage
	{
		TemporaryStorage
		{
			data: [NULL; DEFAULT_STORAGE_SIZE],
			occupied: 0,
			max_usage: 0,
			allocator: Allocator{index: 0, data: [NULL; DEFAULT_ALLOCATOR_SIZE]}
		}
	}

	pub fn reset_storage(&mut self)
	{
		self.occupied = 0;
		self.max_usage = 0;
		self.allocator.index = 0;
	}

	fn bump(&mut self)
	{
		self.allocator.index += 1;
		self.occupied += 1;
	}

	pub fn add_byte(&mut self, data_being_added: u8) -> usize
	{
		let data_at_index = self.allocator.index;
		self.data[self.allocator.index] = data_being_added;
		self.bump();
		return data_at_index;
	}

	pub fn add_byte_vec(&mut self, data_being_added: Vec<u8>) -> usize
	{
		let data_at_index = self.allocator.index;
		for data_item in data_being_added 
		{ 
			self.data[self.allocator.index] = data_item;
			self.bump();
		}

		return data_at_index;
	}
}