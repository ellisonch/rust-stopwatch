extern crate time;
extern crate num;

use std::default::Default;
use std::fmt;
use num::traits::ToPrimitive;
use time::Duration;

#[derive(Clone, Copy)]
pub struct Stopwatch {
	start_time: Option<u64>,
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

impl fmt::Display for Stopwatch {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "{}", self.elapsed())
	}
}

fn current_time() -> u64 {
	time::precise_time_ns()
}

// This only works under the assumption that less than 2^63 ns have passed between t1 and t2 (~292 years)
fn ns_times_to_duration(t1: u64, t2: u64) -> Duration {
	let diff_u = t2 - t1; // works even if there's wraparound
	let diff_i = match diff_u.to_i64() {
		Some(i) => i,
		None => {
			debug_assert!(false, "Stopwatch saw a time of more than 292 years, this probably indicates a bug");
			0
		}
	};
	Duration::nanoseconds(diff_i)
}

impl Stopwatch {
	pub fn new() -> Stopwatch {
		Default::default()
	}
	pub fn start_new() -> Stopwatch {
		let mut sw = Stopwatch::new();
		sw.start();
        sw
	}

	pub fn start(&mut self) {
		self.start_time = Some(current_time());
	}
	pub fn stop(&mut self) {
		self.elapsed = self.elapsed();
		self.start_time = None;
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
		self.start_time.is_some()
	}

	pub fn elapsed(&self) -> Duration {
		match self.start_time {
			Some(t1) => {
				let t2 = current_time();
				let new_duration = ns_times_to_duration(t1, t2);
				new_duration + self.elapsed
			},
			None => {
				self.elapsed
			},
		}
	}
	pub fn elapsed_ms(&self) -> i64 {
		self.elapsed().num_milliseconds()
	}
}
