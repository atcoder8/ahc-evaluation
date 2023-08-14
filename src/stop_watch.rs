//! Provide a structure to measure elapsed time.

use std::time::Instant;

/// Measure the elapsed time.
#[derive(Debug, Clone, Copy)]
pub struct StopWatch {
    /// Instant at the start of the measurement.
    start_instant: Instant,
}

impl Default for StopWatch {
    fn default() -> Self {
        Self::new()
    }
}

impl StopWatch {
    /// Instantiate and measurement is started.
    pub fn new() -> Self {
        Self {
            start_instant: Instant::now(),
        }
    }

    /// Returns the elapsed time in seconds.
    pub fn elapsed_time(&self) -> f64 {
        self.start_instant.elapsed().as_secs_f64()
    }

    /// Return the elapsed time and set the measurement start time to the current time.
    pub fn reset(&mut self) -> f64 {
        let elapsed_time = self.elapsed_time();

        self.start_instant = Instant::now();

        elapsed_time
    }
}
