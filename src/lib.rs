#![feature(duration)]

extern crate time;

use std::time::Duration;
use std::default::Default;
use std::fmt;

#[derive(Clone, Copy)]
pub struct Stopwatch {
	start_time: Option<u64>,
	elapsed: Duration,
}

impl Default for Stopwatch {
	fn default () -> Stopwatch {
		Stopwatch {
			start_time: None,
			elapsed: Duration::new(0, 0),
		}
	}
}

impl fmt::Display for Stopwatch {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		return write!(f, "{}", self.elapsed());
	}
}

fn current_time() -> u64 {
	return time::precise_time_ns();
}

// This only works under the assumption that less than 2^63 ns have passed between t1 and t2 (~292 years)
fn ns_times_to_duration(t1: u64, t2: u64) -> Duration {
    let mut diff = t2.wrapping_sub(t1);
    if (diff as i64) < 0 {
	    debug_assert!(false, "Stopwatch saw a time of more than 292 years, this probably indicates a bug");
        diff = 0;
    }
    return Duration::new(diff / 1_000_000_000, (diff % 1_000_000_000) as u32)
}

#[test]
fn test_ns_times_to_duration() {
	assert_eq!(ns_times_to_duration(100, 1100), Duration::new(0, 1000));
	assert_eq!(ns_times_to_duration(std::u64::MAX-30, std::u64::MAX-10), Duration::new(0, 20));
	assert_eq!(ns_times_to_duration(std::u64::MAX-10, std::u64::MAX), Duration::new(0,10));
	assert_eq!(ns_times_to_duration(std::u64::MAX-10, 0), Duration::new(0, 11));
	assert_eq!(ns_times_to_duration(std::u64::MAX-10, 9), Duration::new(0, 20));
	// assert_eq!(ns_times_to_duration(0, std::u64::MAX-999), Duration::new(0, 0));
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
		self.start_time = Some(current_time());
	}
	pub fn stop(&mut self) {
		self.elapsed = self.elapsed();
		self.start_time = None;
	}
	pub fn reset(&mut self) {
		self.start_time = None;
		self.elapsed = Duration::new(0, 0);
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
				let t2 = current_time();
				let new_duration = ns_times_to_duration(t1, t2);
				return new_duration + self.elapsed;
			},
			None => {
				return self.elapsed;
			},
		}
	}
	pub fn elapsed_ms(&self) -> u64 {
        let eps = self.elapsed();
        eps.secs() * 1_000 + eps.extra_nanos() as u64 / 1_000_000
	}
}
