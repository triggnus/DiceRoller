
use std::sync::{Arc, Mutex};
use std::thread;
use std::thread::JoinHandle;
use rand::Rng;

/// Roll a die with the given number of sides.
pub fn roll(sides: u32) -> f64
{
	rand::thread_rng().gen_range(1..=sides) as f64
}

pub struct DiceRoller
{
	pub(crate) sides: u32,
	pub(crate) times: u32,
	pub(crate) value_a: Arc<Mutex<f64>>,
	pub(crate) value_b: Arc<Mutex<f64>>,
	pub(crate) results: String,
	pub(crate) handle_a: JoinHandle<()>,
	pub(crate) handle_b: JoinHandle<()>,
	pub rolling_a: bool,
	pub rolling_b: bool,
	pub ready_a: bool,
	pub ready_b: bool,
}

impl DiceRoller
{
	pub fn new() -> DiceRoller
	{
		Self
		{
			sides: 0,
			times: 0,
			value_a: Arc::new(Mutex::new(0.0)),
			value_b: Arc::new(Mutex::new(0.0)),
			results: "".to_owned(),
			handle_a: thread::spawn(move || {}),
			handle_b: thread::spawn(move || {}),
			rolling_a: false,
			rolling_b: false,
			ready_a: false,
			ready_b: false,
		}
	}

	pub fn reset(&mut self)
	{
		self.value_a = Arc::new(Mutex::new(0.0));
		self.value_b = Arc::new(Mutex::new(0.0));
		self.results = "".to_owned();
		self.handle_a = thread::spawn(move || {});
		self.handle_b = thread::spawn(move || {});
		self.rolling_a = false;
		self.rolling_b = false;
		self.ready_a = false;
		self.ready_b = false;
	}

	/// Start the process for running two dice.
	pub(crate) fn start_rolling_a(&mut self) -> JoinHandle<()>
	{
		let value = Arc::clone(&self.value_a);
		let sides = self.sides;
		let times = self.times;
		self.rolling_a = true;

		let handle = thread::spawn(move || {

			let mut roll_total = value.lock().unwrap();

			for _ in 0..times
			{
				*roll_total += roll(sides) + roll(sides);
			}
			//println!("Method A - Done.");
		});

		//println!("Method A - Loaded.");
		handle
	}

	/// Start the process for rolling one die and multiplying by two.
	pub(crate) fn start_rolling_b(&mut self) -> JoinHandle<()>
	{
		let value = Arc::clone(&self.value_b);
		let sides = self.sides;
		let times = self.times;
		self.rolling_b = true;

		let handle = thread::spawn(move || {
			let mut roll_total = value.lock().unwrap();

			for _ in 0..times
			{
				*roll_total += 2.0 * roll(sides);
			}
			//println!("Method B - Done.");
		});

		//println!("Method B - Loaded.");
		handle
	}

	pub(crate) fn result_a(&self) -> f64
	{
		let total = self.value_a.lock().unwrap().clone();
		return total / self.times as f64;
	}

	pub(crate) fn result_b(&self) -> f64
	{
		let total = self.value_b.lock().unwrap().clone();
		return total / self.times as f64;
	}
}