extern crate time;
extern crate stopwatch;

use stopwatch::{Stopwatch};

static SLEEP_MS: u64 = 50;
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
	sleep_ms(SLEEP_MS);
	assert_sw_near(sw, SLEEP_MS);
}

#[test]
fn stop() {
	let mut sw = Stopwatch::start_new();
	sleep_ms(SLEEP_MS);
	sw.stop();
	assert_sw_near(sw, SLEEP_MS);

	sleep_ms(SLEEP_MS);
	assert_sw_near(sw, SLEEP_MS);
}

#[test]
fn resume_once() {
	let mut sw = Stopwatch::start_new();
	sleep_ms(SLEEP_MS);
	sw.stop();
	assert_sw_near(sw, SLEEP_MS);
	sw.start();
	sleep_ms(SLEEP_MS);
	assert_sw_near(sw, 2 * SLEEP_MS);
}

#[test]
fn resume_twice() {
	let mut sw = Stopwatch::start_new();
	sleep_ms(SLEEP_MS);
	sw.stop();
	assert_sw_near(sw, SLEEP_MS);
	sw.start();
	sleep_ms(SLEEP_MS);
	sw.stop();
	assert_sw_near(sw, 2 * SLEEP_MS);
	sw.start();
	sleep_ms(SLEEP_MS);
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
	sleep_ms(SLEEP_MS);
	sw.restart();
	sleep_ms(SLEEP_MS);
	assert_sw_near(sw, SLEEP_MS);
}

#[test]
fn reset() {
	let mut sw = Stopwatch::start_new();
	sleep_ms(SLEEP_MS);
	sw.reset();
	assert!(!sw.is_running());
	assert_eq!(sw.elapsed_ms(), 0);
}

/////////////// helpers

fn sleep_ms(ms: u64) {
	std::thread::sleep_ms(ms as u32)
}

fn assert_near(x: u64, y: u64, tolerance: u64) {
	let diff = x - y;
	if diff > tolerance {
		panic!("Expected {:?}, got {:?}", x, y);
	}
}

fn assert_sw_near(sw: Stopwatch, elapsed: u64) {
	let tolerance_value = (TOLERANCE_PERCENTAGE * elapsed as f64) as u64;
	assert_near(elapsed, sw.elapsed_ms(), tolerance_value);
}
