extern crate time;

use std::time::Duration;
use std::default::Default;
use std::fmt;
use time::{Timespec};

#[deriving(Copy)]
pub struct Stopwatch {
	start_time: Option<Timespec>,
	elapsed: Duration,
}

impl Default for Stopwatch {
	fn default () -> Stopwatch {
		Stopwatch {
			start_time: None,
			elapsed: Duration::zero(),
		}
	}
}

impl fmt::Show for Stopwatch {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		return write!(f, "{}", self.elapsed());
	}
}

fn current_time() -> Timespec {
	return time::now_utc().to_timespec();
}

impl Stopwatch {
	pub fn new() -> Stopwatch {
		let sw: Stopwatch = Default::default();
		return sw;
	}
	pub fn start_new() -> Stopwatch {
		let mut sw = Stopwatch::new();
		let time = current_time();
		sw.start_time = Some(time);
		return sw;
	}

	pub fn start(&mut self) {
		let time = current_time();
		self.start_time = Some(time);
	}
	pub fn stop(&mut self) {
		match self.start_time {
			Some(t1) => {
				let t2 = current_time();
				self.elapsed = self.elapsed + (t2 - t1);
				self.start_time = None;
			},
			None => {
			},
		}
	}
	pub fn reset(&mut self) {
		self.start_time = None;
		self.elapsed = Duration::zero();
	}
	pub fn restart(&mut self) {
		self.reset();
		self.start();
	}

	pub fn is_running(&self) -> bool {
		return match self.start_time {
			Some(_) => true,
			None => false,
		};
	}

	pub fn elapsed(&self) -> Duration {
		match self.start_time {
			Some(t1) => {
				let t2 = current_time();
				return (t2 - t1) + self.elapsed;
			},
			None => {
				return self.elapsed;
			},
		}
	}
	pub fn elapsed_ms(&self) -> i64 {
		return self.elapsed().num_milliseconds();
	}
}
