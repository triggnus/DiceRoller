#![allow(dead_code)]

use std::collections::HashMap;

pub struct Stats
{
	data: HashMap<i32, u32>,
}

impl Stats
{
	pub fn new() -> Stats
	{
		Stats { data: HashMap::new() }
	}

	/// Add a roll of some value and increment the count of that roll
	pub fn add(&mut self, roll: i32)
	{
		if self.data.contains_key(&roll)
		{
			self.data.insert(roll, self.data[&roll] + 1);
		}
		else
		{
			self.data.insert(roll, 1);
		}
	}

	pub fn distribution(&self) -> Vec<(i32, u32)>
	{
		let mut result = Vec::new();

		for rolls in self.data.keys()
		{
			result.push((*rolls, *self.data.get(rolls).unwrap()));
		}

		result
	}
}
