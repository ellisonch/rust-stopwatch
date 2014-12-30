extern crate time;

extern crate stopwatch;
use stopwatch::{Stopwatch};

use std::io::timer;
use std::time::Duration;

#[test]
fn it_works() {
	let mut sw = Stopwatch::start_new();

	timer::sleep(Duration::milliseconds(200));

	sw.stop();

	println!("{}", sw.elapsed_ms());
}

#[test]
fn is_running() {
	let mut sw = Stopwatch::new();
	assert!(!sw.is_running());
	sw.start();
	assert!(sw.is_running());
	sw.stop();
	assert!(!sw.is_running());
}

#[test]
fn restart() {
	let mut sw = Stopwatch::start_new();
	timer::sleep(Duration::milliseconds(50));
	sw.restart();
	timer::sleep(Duration::milliseconds(50));
	println!("{}", sw.elapsed_ms());
}
