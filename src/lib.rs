use std::default::Default;
use std::fmt;
use std::time::{Duration, Instant};

#[derive(Clone, Copy)]
pub struct Stopwatch {
	/// The time the stopwatch was started last, if ever.
	start_time: Option<Instant>,
	/// The time the stopwatch was split last, if ever.
	split_time: Option<Instant>,
	/// The time elapsed while the stopwatch was running (between start() and stop()).
	elapsed: Duration,
}

impl Default for Stopwatch {
	fn default() -> Stopwatch {
		Stopwatch {
			start_time: None,
			split_time: None,
			elapsed: Duration::from_secs(0),
		}
	}
}

impl fmt::Display for Stopwatch {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut ms = self.elapsed_ms();
        let s: i64;
        let m: i64;
        let h: i64;
        let mut buf = String::new();

        s = ms  / 1000;
        ms = ms % 1000;
        buf.insert_str(0, &format!("{}ms", ms));
        if s > 0{
            buf.insert_str(0, &format!("{}s ", s));
        } else {
            return buf.fmt(f);
        }

        m = s  / 60;
        if m > 0{
            buf.insert_str(0, &format!("{}m ", m));
        } else {
            return buf.fmt(f);
        }

        h = m  / 60;
        if h > 0{
            buf.insert_str(0, &format!("{}h ", h));
        } else {
            return buf.fmt(f);
        }

        return buf.fmt(f);
	}
}

impl Stopwatch {
	/// Returns a new stopwatch.
	pub fn new() -> Stopwatch {
		let sw: Stopwatch = Default::default();
		return sw;
	}

	/// Returns a new stopwatch which will immediately be started.
	pub fn start_new() -> Stopwatch {
		let mut sw = Stopwatch::new();
		sw.start();
		return sw;
	}

	/// Starts the stopwatch.
	pub fn start(&mut self) {
		self.start_time = Some(Instant::now());
	}

	/// Stops the stopwatch.
	pub fn stop(&mut self) {
		self.elapsed = self.elapsed();
		self.start_time = None;
		self.split_time = None;
	}

	/// Resets all counters and stops the stopwatch.
	pub fn reset(&mut self) {
		self.elapsed = Duration::from_secs(0);
		self.start_time = None;
		self.split_time = None;
	}

	/// Resets and starts the stopwatch again.
	pub fn restart(&mut self) {
		self.reset();
		self.start();
	}

	/// Returns whether the stopwatch is running.
	pub fn is_running(&self) -> bool {
		return self.start_time.is_some();
	}

	/// Returns the elapsed time since the start of the stopwatch.
	pub fn elapsed(&self) -> Duration {
		match self.start_time {
			// stopwatch is running
			Some(t1) => {
				return t1.elapsed() + self.elapsed;
			}
			// stopwatch is not running
			None => {
				return self.elapsed;
			}
		}
	}

	/// Returns elapsed time since the start of stopwatch in nanoseconds
	pub fn elapsed_ns(&self) -> u64 {
		self.elapsed().subsec_nanos() as u64
	}

	/// Returns the elapsed time since the start of the stopwatch in milliseconds.
	pub fn elapsed_ms(&self) -> i64 {
		let dur = self.elapsed();
		return (dur.as_secs() * 1000 + (dur.subsec_nanos() / 1000000) as u64) as i64;
	}

	/// Returns elapsed time since the start of stopwatch in seconds
	pub fn elapsed_sec(&self) -> f64 {
        ((self.elapsed_ms() * 1000) as f64).round() / 1.0e6
	}

    /// Returns elapsed time since the start of stopwatch in seconds
    pub fn elapsed_min(&self) -> f64 {
        (self.elapsed_sec() * 1000.0 / 60.0).round() / 1.0e3
    }

    /// Returns elapsed time since the start of stopwatch in hours
	pub fn elapsed_hour(&self) -> f64 {
        (self.elapsed_min() * 1000.0 / 60.0).round() / 1.0e3
    }

	/// Returns the elapsed time since last split or start/restart.
	///
	/// If the stopwatch is in stopped state this will always return a zero Duration.
	pub fn elapsed_split(&mut self) -> Duration {
		match self.start_time {
			// stopwatch is running
			Some(start) => {
				let res = match self.split_time {
					Some(split) => split.elapsed(),
					None => start.elapsed(),
				};
				self.split_time = Some(Instant::now());
				res
			}
			// stopwatch is not running
			None => Duration::from_secs(0),
		}
	}

    /// Returns elapsed time since last split or start/restart in nanoseconds.
    ///
	/// If the stopwatch is in stopped state this will always return zero.
    pub fn elapsed_split_ns(&mut self) -> u64 {
        self.elapsed_split().subsec_nanos() as u64
    }

	/// Returns the elapsed time since last split or start/restart in milliseconds.
	///
	/// If the stopwatch is in stopped state this will always return zero.
	pub fn elapsed_split_ms(&mut self) -> i64 {
		let dur = self.elapsed_split();
		return (dur.as_secs() * 1000 + (dur.subsec_nanos() / 1_000_000) as u64) as i64;
	}

    /// Returns elapsed time since last split or start/restart in seconds
    ///
	/// If the stopwatch is in stopped state this will always return zero.
    pub fn elapsed_split_sec(&mut self) -> f64 {
        ((self.elapsed_split_ms() * 1000) as f64).round() / 1.0e6
    }

    /// Returns elapsed time since last split or start/restart in minutes
    ///
	/// If the stopwatch is in stopped state this will always return zero.
    pub fn elapsed_split_min(&mut self) -> f64 {
        (self.elapsed_split_sec() * 1000.0 / 60.0).round() / 1.0e3
    }

    /// Returns elapsed time since last split or start/restart in hours
    ///
	/// If the stopwatch is in stopped state this will always return zero.
    pub fn elapsed_split_hour(&mut self) -> f64 {
        (self.elapsed_split_min() * 1000.0 / 60.0).round() / 1.0e3
    }
}
