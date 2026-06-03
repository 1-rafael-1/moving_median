//! # moving median
//!
//! A simple no-std moving median filter implementation with a fixed-size buffer.
//! The buffer is used to store the last N measurements, where N is the size of the buffer.
//! The median is calculated by sorting the values in the buffer and taking the middle value.
//! If the number of values is even, the median is the average of the two middle values.
//! If the number of values is odd, the median is the middle value.
//!
//! ## Example
//!
//! ```
//! use moving_median::MovingMedian;
//!
//! let mut filter_f32 = MovingMedian::<f32, 3>::new();
//! filter_f32.add_value(42.0);
//! filter_f32.add_value(43.0);
//! filter_f32.add_value(41.0);
//! assert_eq!(filter_f32.median(), Some(42.0_f32));
//! ```
//!
//! ```
//! use moving_median::MovingMedian;
//!
//! let mut filter_f64 = MovingMedian::<f64, 3>::new();
//! filter_f64.add_value(42.0);
//! filter_f64.add_value(43.0);
//! filter_f64.add_value(41.0);
//! assert_eq!(filter_f64.median(), Some(42.0_f64));
//! ```
//!
//! ```
//! use moving_median::MovingMedian;
//!
//! let mut filter_f64 = MovingMedian::<f64, 3>::new();
//! filter_f64.add_value(42.0);
//! filter_f64.add_value(43.0);
//! filter_f64.add_value(41.0);
//! filter_f64.clear();
//! assert_eq!(filter_f64.median(), None);
//! ```

#![no_std]

use core::cmp::PartialOrd;
use core::ops::{Add, Div};

/// A simple no-std moving median filter implementation with a fixed-size buffer.
///
/// The buffer is used to store the last N measurements, where N is the size of the buffer.
/// The median is calculated by sorting the values in the buffer and taking the middle value.
/// If the number of values is even, the median is the average of the two middle values.
/// If the number of values is odd, the median is the middle value.
///
/// # Type parameter
///
/// `T` can be any type satisfying the trait bounds, including `f32`, `f64`, and integer types
/// such as `i16`, `i32`, `u32`, etc. Note that `i8` is not supported because it does not
/// implement `From<u8>`.
///
/// When using integer types, the even-count median is computed using integer division, which
/// truncates toward zero. For example, the median of `[10, 11]` is `10`, not `10.5`.
/// If this is not acceptable, use `f32` or `f64` instead.
#[must_use]
pub struct MovingMedian<T, const N: usize> {
    // Fixed-size buffer to hold the measurements
    buffer: [T; N],
    // Current index in the buffer
    index: usize,
    // Number of values added (up to N)
    count: usize,
}

impl<T, const N: usize> Default for MovingMedian<T, N>
where
    T: Copy + PartialOrd + Add<Output = T> + Div<Output = T> + From<u8> + Default,
{
    fn default() -> Self {
        Self::new()
    }
}

impl<T, const N: usize> MovingMedian<T, N>
where
    T: Copy + PartialOrd + Add<Output = T> + Div<Output = T> + From<u8> + Default,
{
    /// Create a new moving median filter with a fixed-size buffer of size N.
    pub fn new() -> Self {
        // catch N being 0 at compile time, which would later fail at runtime
        const { assert!(N > 0, "MovingMedian: window size N must be greater than 0") };

        Self {
            buffer: [T::default(); N],
            index: 0,
            count: 0,
        }
    }

    /// Add a new measurement to the buffer.
    /// If the buffer is full, the oldest value will be replaced.
    /// The buffer will always contain the last N measurements.
    /// The count will be incremented up to N.
    pub const fn add_value(&mut self, value: T) {
        // Add the new value to the buffer
        self.buffer[self.index] = value;
        // Move to the next index, wrapping around if necessary
        self.index = (self.index + 1) % N;
        // Increment the count up to N
        if self.count < N {
            self.count += 1;
        }
    }

    /// Calculate the median of the values in the buffer.
    ///
    /// Returns `None` if no values have been added yet.
    ///
    /// The median is the middle value when the values are sorted in ascending order.
    /// If the number of values is odd, the median is the middle value.
    /// If the number of values is even, the median is the average of the two middle values.
    ///
    /// # Note on integer types
    ///
    /// For integer `T`, the even-count average is computed with integer division and truncates
    /// toward zero. The median of `[10, 11]` is `10`, not `10.5`.
    #[must_use]
    pub fn median(&self) -> Option<T> {
        // If no values have been added, return None
        if self.count == 0 {
            return None;
        }

        // Find the median
        let sorted = self.sort();
        Some(self.median_from_sorted(&sorted))
    }

    /// clear the buffer
    /// The buffer will be filled with default values
    /// The count and index will be set to 0
    pub fn clear(&mut self) {
        self.buffer = [T::default(); N];
        self.count = 0;
        self.index = 0;
    }

    /// Get the number of values currently in the buffer (up to N).
    #[must_use]
    pub const fn len(&self) -> usize {
        self.count
    }

    /// Check if the buffer is empty (no values added).
    #[must_use]
    pub const fn is_empty(&self) -> bool {
        self.count == 0
    }

    /// Check if the buffer is full (contains N values).
    #[must_use]
    pub const fn is_full(&self) -> bool {
        self.count == N
    }

    /// Get the capacity of the buffer (maximum number of values it can hold).
    #[must_use]
    pub const fn capacity(&self) -> usize {
        N
    }

    /// Returns the min value in the current buffer
    #[must_use]
    pub fn min(&self) -> Option<T> {
        if self.count == 0 {
            return None;
        }
        let mut min = self.buffer[0];
        for i in 1..self.count {
            if self.buffer[i] < min {
                min = self.buffer[i];
            }
        }
        Some(min)
    }

    /// Returns the max value in the current buffer
    #[must_use]
    pub fn max(&self) -> Option<T> {
        if self.count == 0 {
            return None;
        }
        let mut max = self.buffer[0];
        for i in 1..self.count {
            if self.buffer[i] > max {
                max = self.buffer[i];
            }
        }
        Some(max)
    }

    /// Returns `(min, median, max)` of the values currently in the buffer, or `None` if empty.
    ///
    /// Performs a single sort, making this more efficient than calling `min()`, `median()`,
    /// and `max()` individually.
    ///
    /// # Note on integer types
    ///
    /// The median value follows the same integer truncation behaviour as [`Self::median()`].
    #[must_use]
    pub fn stats(&self) -> Option<(T, T, T)> {
        if self.count == 0 {
            return None;
        }
        let sorted = self.sort();
        let min = sorted[0];
        let max = sorted[self.count - 1];
        let median = self.median_from_sorted(&sorted);
        Some((min, median, max))
    }

    /// Compute median from an already-sorted buffer slice.
    fn median_from_sorted(&self, sorted: &[T; N]) -> T {
        if self.count.is_multiple_of(2) {
            (sorted[self.count / 2 - 1] + sorted[self.count / 2]) / T::from(2)
        } else {
            sorted[self.count / 2]
        }
    }

    /// Create a copy of the buffer and sort it
    fn sort(&self) -> [T; N] {
        let mut sorted = self.buffer;
        sorted[..self.count]
            .sort_unstable_by(|a, b| a.partial_cmp(b).unwrap_or(core::cmp::Ordering::Equal));
        sorted
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[allow(clippy::float_cmp)]
    fn median_is_none_when_no_values_added() {
        let filter = MovingMedian::<f64, 3>::new();
        assert_eq!(filter.median(), None);
    }

    #[test]
    #[allow(clippy::float_cmp)]
    fn median_is_value_when_one_value_added() {
        let mut filter = MovingMedian::<f64, 3>::new();
        filter.add_value(42.0);
        assert_eq!(filter.median(), Some(42.0));
    }

    #[test]
    #[allow(clippy::float_cmp)]
    fn median_is_average_of_two_values_when_two_values_added() {
        let mut filter = MovingMedian::<f64, 2>::new();
        filter.add_value(42.0);
        filter.add_value(43.0);
        assert_eq!(filter.median(), Some(42.5));
    }

    #[test]
    #[allow(clippy::float_cmp)]
    fn median_is_middle_value_when_three_values_added() {
        let mut filter = MovingMedian::<f64, 3>::new();
        filter.add_value(42.0);
        filter.add_value(43.0);
        filter.add_value(41.0);
        assert_eq!(filter.median(), Some(42.0));
    }

    #[test]
    #[allow(clippy::float_cmp)]
    fn median_is_average_of_two_middle_values_when_four_values_added() {
        let mut filter = MovingMedian::<f64, 4>::new();
        filter.add_value(42.0);
        filter.add_value(43.0);
        filter.add_value(41.0);
        filter.add_value(44.0);
        assert_eq!(filter.median(), Some(42.5));
    }

    #[test]
    #[allow(clippy::float_cmp)]
    fn median_is_midlle_value_of_n_values_when_more_than_n_values_added() {
        let mut filter = MovingMedian::<f64, 3>::new();
        filter.add_value(42.0); // should be pushed out
        filter.add_value(44.0);
        filter.add_value(43.0); // should be the median
        filter.add_value(41.0);
        assert_eq!(filter.median(), Some(43.0));
    }

    #[test]
    #[allow(clippy::float_cmp)]
    fn median_is_none_when_cleared() {
        let mut filter = MovingMedian::<f64, 3>::new();
        filter.add_value(42.0);
        filter.add_value(43.0);
        filter.add_value(41.0);
        filter.clear();
        assert_eq!(filter.median(), None);
    }

    // --- len / is_empty / is_full / capacity ---

    #[test]
    fn len_is_zero_when_no_values_added() {
        let filter = MovingMedian::<f64, 3>::new();
        assert_eq!(filter.len(), 0);
    }

    #[test]
    fn len_grows_as_values_are_added() {
        let mut filter = MovingMedian::<f64, 3>::new();
        filter.add_value(1.0);
        assert_eq!(filter.len(), 1);
        filter.add_value(2.0);
        assert_eq!(filter.len(), 2);
        filter.add_value(3.0);
        assert_eq!(filter.len(), 3);
    }

    #[test]
    fn len_does_not_exceed_capacity() {
        let mut filter = MovingMedian::<f64, 3>::new();
        filter.add_value(1.0);
        filter.add_value(2.0);
        filter.add_value(3.0);
        filter.add_value(4.0);
        assert_eq!(filter.len(), 3);
    }

    #[test]
    fn is_empty_is_true_when_no_values_added() {
        let filter = MovingMedian::<f64, 3>::new();
        assert!(filter.is_empty());
    }

    #[test]
    fn is_empty_is_false_after_value_added() {
        let mut filter = MovingMedian::<f64, 3>::new();
        filter.add_value(1.0);
        assert!(!filter.is_empty());
    }

    #[test]
    fn is_empty_is_true_after_clear() {
        let mut filter = MovingMedian::<f64, 3>::new();
        filter.add_value(1.0);
        filter.clear();
        assert!(filter.is_empty());
    }

    #[test]
    fn is_full_is_false_when_not_enough_values_added() {
        let mut filter = MovingMedian::<f64, 3>::new();
        filter.add_value(1.0);
        filter.add_value(2.0);
        assert!(!filter.is_full());
    }

    #[test]
    fn is_full_is_true_when_buffer_full() {
        let mut filter = MovingMedian::<f64, 3>::new();
        filter.add_value(1.0);
        filter.add_value(2.0);
        filter.add_value(3.0);
        assert!(filter.is_full());
    }

    #[test]
    fn is_full_remains_true_after_window_rolls() {
        let mut filter = MovingMedian::<f64, 3>::new();
        filter.add_value(1.0);
        filter.add_value(2.0);
        filter.add_value(3.0);
        filter.add_value(4.0);
        assert!(filter.is_full());
    }

    #[test]
    fn capacity_equals_n() {
        let filter = MovingMedian::<f64, 5>::new();
        assert_eq!(filter.capacity(), 5);
    }

    // --- min ---

    #[test]
    fn min_is_none_when_empty() {
        let filter = MovingMedian::<f64, 3>::new();
        assert_eq!(filter.min(), None);
    }

    #[test]
    #[allow(clippy::float_cmp)]
    fn min_returns_smallest_value() {
        let mut filter = MovingMedian::<f64, 3>::new();
        filter.add_value(42.0);
        filter.add_value(41.0);
        filter.add_value(43.0);
        assert_eq!(filter.min(), Some(41.0));
    }

    #[test]
    #[allow(clippy::float_cmp)]
    fn min_reflects_rolling_window() {
        let mut filter = MovingMedian::<f64, 3>::new();
        filter.add_value(41.0); // will be pushed out
        filter.add_value(43.0);
        filter.add_value(44.0);
        filter.add_value(45.0);
        assert_eq!(filter.min(), Some(43.0));
    }

    // --- max ---

    #[test]
    fn max_is_none_when_empty() {
        let filter = MovingMedian::<f64, 3>::new();
        assert_eq!(filter.max(), None);
    }

    #[test]
    #[allow(clippy::float_cmp)]
    fn max_returns_largest_value() {
        let mut filter = MovingMedian::<f64, 3>::new();
        filter.add_value(42.0);
        filter.add_value(41.0);
        filter.add_value(43.0);
        assert_eq!(filter.max(), Some(43.0));
    }

    #[test]
    #[allow(clippy::float_cmp)]
    fn max_reflects_rolling_window() {
        let mut filter = MovingMedian::<f64, 3>::new();
        filter.add_value(45.0); // will be pushed out
        filter.add_value(41.0);
        filter.add_value(42.0);
        filter.add_value(43.0);
        assert_eq!(filter.max(), Some(43.0));
    }

    // --- stats ---

    #[test]
    fn stats_is_none_when_empty() {
        let filter = MovingMedian::<f64, 3>::new();
        assert_eq!(filter.stats(), None);
    }

    #[test]
    #[allow(clippy::float_cmp)]
    fn stats_returns_min_median_max() {
        let mut filter = MovingMedian::<f64, 3>::new();
        filter.add_value(42.0);
        filter.add_value(41.0);
        filter.add_value(43.0);
        assert_eq!(filter.stats(), Some((41.0, 42.0, 43.0)));
    }

    #[test]
    #[allow(clippy::float_cmp)]
    fn stats_with_even_count_averages_median() {
        let mut filter = MovingMedian::<f64, 4>::new();
        filter.add_value(41.0);
        filter.add_value(42.0);
        filter.add_value(43.0);
        filter.add_value(44.0);
        assert_eq!(filter.stats(), Some((41.0, 42.5, 44.0)));
    }

    #[test]
    #[allow(clippy::float_cmp)]
    fn stats_reflects_rolling_window() {
        let mut filter = MovingMedian::<f64, 3>::new();
        filter.add_value(99.0); // will be pushed out
        filter.add_value(41.0);
        filter.add_value(42.0);
        filter.add_value(43.0);
        assert_eq!(filter.stats(), Some((41.0, 42.0, 43.0)));
    }

    // --- integer types ---

    #[test]
    fn integer_odd_count_median_is_exact() {
        let mut filter = MovingMedian::<i32, 3>::new();
        filter.add_value(10);
        filter.add_value(20);
        filter.add_value(30);
        assert_eq!(filter.median(), Some(20));
    }

    #[test]
    fn integer_even_count_median_truncates() {
        let mut filter = MovingMedian::<i32, 2>::new();
        filter.add_value(10);
        filter.add_value(11);
        // integer division: (10 + 11) / 2 = 10, not 10.5
        assert_eq!(filter.median(), Some(10));
    }

    // --- Default ---

    #[test]
    fn default_produces_empty_filter() {
        let filter = MovingMedian::<f64, 3>::default();
        assert!(filter.is_empty());
        assert_eq!(filter.median(), None);
    }
}
