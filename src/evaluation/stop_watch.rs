//! Provides a structure to measure elapsed time.

use std::time::Instant;

/// Measures the elapsed time.
#[derive(Debug, Clone, Copy)]
pub struct Stopwatch {
    /// Instant at the start of the measurement.
    start_instant: Instant,
}

impl Stopwatch {
    /// Instantiates and measurement is started.
    pub fn start() -> Self {
        Self {
            start_instant: Instant::now(),
        }
    }

    /// Returns the elapsed time in seconds.
    pub fn elapsed_time(&self) -> f64 {
        self.start_instant.elapsed().as_secs_f64()
    }
}
