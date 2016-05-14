extern crate num;

use std::default::Default;
use std::fmt;
use std::time::{Duration, Instant};

#[derive(Clone, Copy)]
pub struct Stopwatch {
	start_time: Option<Instant>,
	elapsed: Duration,
}

impl Default for Stopwatch {
	fn default () -> Stopwatch {
		Stopwatch {
			start_time: None,
			elapsed: Duration::from_secs(0),
		}
	}
}

impl fmt::Display for Stopwatch {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		return write!(f, "{}ms", self.elapsed_ms());
	}
}

impl Stopwatch {
	pub fn new() -> Stopwatch {
		let sw: Stopwatch = Default::default();
		return sw;
	}
	pub fn start_new() -> Stopwatch {
		let mut sw = Stopwatch::new();
		sw.start();
		return sw;
	}

	pub fn start(&mut self) {
		self.start_time = Some(Instant::now());
	}
	pub fn stop(&mut self) {
		self.elapsed = self.elapsed();
		self.start_time = None;
	}
	pub fn reset(&mut self) {
		self.elapsed = Duration::from_secs(0);
		self.start_time = None;
	}
	pub fn restart(&mut self) {
		self.reset();
		self.start();
	}

	pub fn is_running(&self) -> bool {
		return self.start_time.is_some();
	}

	pub fn elapsed(&self) -> Duration {
		match self.start_time {
			Some(t1) => {
				return t1.elapsed() + self.elapsed;
			},
			None => {
				return self.elapsed;
			},
		}
	}
	pub fn elapsed_ms(&self) -> i64 {
		let dur = self.elapsed();
		return (dur.as_secs() * 1000 + (dur.subsec_nanos() / 1000000) as u64) as i64;
	}
}
