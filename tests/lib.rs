extern crate time;
extern crate stopwatch;

use stopwatch::{Stopwatch};
use std::num::SignedInt;
use std::old_io::timer;
use std::time::Duration;

static SLEEP_MS: i64 = 50;
static TOLERANCE_PERCENTAGE: f64 = 0.3;

#[test]
fn repeated_stops() {
	let mut sw = Stopwatch::start_new();
	for _ in (0..1000i32) {
		sw.stop();
		sw.start();
	}
	assert_sw_near(sw, 0);
}

#[test]
fn elapsed_none() {
	let sw = Stopwatch::new();
	assert_eq!(sw.elapsed_ms(), 0);
}

#[test]
fn elapsed_ms() {
	let sw = Stopwatch::start_new();
	timer::sleep(Duration::milliseconds(SLEEP_MS));
	assert_sw_near(sw, SLEEP_MS);
}

#[test]
fn stop() {
	let mut sw = Stopwatch::start_new();
	timer::sleep(Duration::milliseconds(SLEEP_MS));
	sw.stop();
	assert_sw_near(sw, SLEEP_MS);

	timer::sleep(Duration::milliseconds(SLEEP_MS));
	assert_sw_near(sw, SLEEP_MS);
}

#[test]
fn resume_once() {
	let mut sw = Stopwatch::start_new();
	timer::sleep(Duration::milliseconds(SLEEP_MS));
	sw.stop();
	assert_sw_near(sw, SLEEP_MS);
	sw.start();
	timer::sleep(Duration::milliseconds(SLEEP_MS));
	assert_sw_near(sw, 2 * SLEEP_MS);
}

#[test]
fn resume_twice() {
	let mut sw = Stopwatch::start_new();
	timer::sleep(Duration::milliseconds(SLEEP_MS));
	sw.stop();
	assert_sw_near(sw, SLEEP_MS);
	sw.start();
	timer::sleep(Duration::milliseconds(SLEEP_MS));
	sw.stop();
	assert_sw_near(sw, 2 * SLEEP_MS);
	sw.start();
	timer::sleep(Duration::milliseconds(SLEEP_MS));
	assert_sw_near(sw, 3 * SLEEP_MS);
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
	timer::sleep(Duration::milliseconds(SLEEP_MS));
	sw.restart();
	timer::sleep(Duration::milliseconds(SLEEP_MS));
	assert_sw_near(sw, SLEEP_MS);
}

#[test]
fn reset() {
	let mut sw = Stopwatch::start_new();
	timer::sleep(Duration::milliseconds(SLEEP_MS));
	sw.reset();
	assert!(!sw.is_running());
	assert_eq!(sw.elapsed_ms(), 0);
}



/////////////// helpers

fn assert_near(x: i64, y: i64, tolerance: i64) {
	let diff = (x - y).abs();
	if diff > tolerance {
		panic!("Expected {:?}, got {:?}", x, y);
	}
}

fn assert_sw_near(sw: Stopwatch, elapsed: i64) {
	let tolerance_value = (TOLERANCE_PERCENTAGE * elapsed as f64) as i64;
	assert_near(elapsed, sw.elapsed_ms(), tolerance_value);
}