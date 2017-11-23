extern crate num;
extern crate stopwatch;

use std::time::Duration;
use stopwatch::{Stopwatch};

static SLEEP_MS: i64 = 50;
static TOLERANCE_PERCENTAGE: f64 = 0.3;

#[test]
fn repeated_stops() {
	let mut sw = Stopwatch::start_new();
	for _ in 0..1000i32 {
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
fn elapsed_ns(){
	let sw = Stopwatch::start_new();
	sleep_ms(2);
	assert!(sw.elapsed_ns() > 1000000);
}

#[test]
fn elapsed_sec(){
    let sw = Stopwatch::start_new();
    sleep_ms(2);
    assert!(sw.elapsed_sec() >= 0.001 || sw.elapsed_sec() <= 0.002);
}

#[test]
fn elapsed_min(){
    let sw = Stopwatch::start_new();
    sleep_ms(3000);
    assert!(sw.elapsed_min() >= 0.05 - 0.002 || sw.elapsed_min() <= 0.05 + 0.002);
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

#[test]
fn split_on_started_watch() {
	let mut sw = Stopwatch::start_new();
	sleep_ms(SLEEP_MS);
	assert_split_near(&mut sw, SLEEP_MS);
	sleep_ms(SLEEP_MS);
	assert_split_near(&mut sw, SLEEP_MS);

	assert_sw_near(sw, 2 * SLEEP_MS);
}

#[test]
fn split_on_resumed_watch() {
	let mut sw = Stopwatch::new();
	sleep_ms(SLEEP_MS);
	assert_split_near(&mut sw, 0);
	sleep_ms(SLEEP_MS);
	assert_split_near(&mut sw, 0);
	sw.start();
	sleep_ms(SLEEP_MS);
	assert_split_near(&mut sw, SLEEP_MS);
	sleep_ms(SLEEP_MS);
	assert_split_near(&mut sw, SLEEP_MS);
	sw.stop();
	sleep_ms(SLEEP_MS);
	assert_split_near(&mut sw, 0);

	assert_sw_near(sw, 2 * SLEEP_MS);
}

#[test]
fn split_after_reset() {
	let mut sw = Stopwatch::start_new();
	sleep_ms(SLEEP_MS);
	assert_split_near(&mut sw, SLEEP_MS);
	sw.reset();
	sleep_ms(SLEEP_MS);
	assert_split_near(&mut sw, 0);
}

#[test]
fn split_after_restart() {
	let mut sw = Stopwatch::start_new();
	sleep_ms(SLEEP_MS);
	assert_split_near(&mut sw, SLEEP_MS);
	sw.restart();
	sleep_ms(SLEEP_MS);
	assert_split_near(&mut sw, SLEEP_MS);
}



/////////////// helpers

fn sleep_ms(ms: i64) {
	use num::ToPrimitive;
	std::thread::sleep(Duration::from_millis(ms.to_u64().unwrap()))
}

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

fn assert_split_near(sw: &mut Stopwatch, elapsed: i64) {
	let tolerance_value = (TOLERANCE_PERCENTAGE * elapsed as f64) as i64;
	assert_near(elapsed, sw.elapsed_split_ms(), tolerance_value);
}