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
//! assert_eq!(filter_f32.median(), 42.0);
//! ```
//!
//! ```
//! use moving_median::MovingMedian;
//!
//! let mut filter_f64 = MovingMedian::<f64, 3>::new();
//! filter_f64.add_value(42.0);
//! filter_f64.add_value(43.0);
//! filter_f64.add_value(41.0);
//! assert_eq!(filter_f64.median(), 42.0);
//! ```
//!
//! //!
//! ```
//! use moving_median::MovingMedian;
//!
//! let mut filter_f64 = MovingMedian::<f64, 3>::new();
//! filter_f64.add_value(42.0);
//! filter_f64.add_value(43.0);
//! filter_f64.add_value(41.0);
//! filter_f64.clear();
//! assert_eq!(filter_f64.median(), 0.0);
//! ```

#![no_std]

use core::cmp::PartialOrd;
use core::ops::{Add, Div};

/// A simple no-std moving median filter implementation with a fixed-size buffer. The buffer is used to store the last N measurements, where N is the size of the buffer.
/// The median is calculated by sorting the values in the buffer and taking the middle value. If the number of values is even, the median is the average of the two middle values. If the number of values is odd, the median is the middle value.
pub struct MovingMedian<T, const N: usize> {
    // Fixed-size buffer to hold the measurements
    buffer: [T; N],
    // Current index in the buffer
    index: usize,
    // Number of values added (up to N)
    count: usize,
}

impl<T, const N: usize> MovingMedian<T, N>
where
    T: Copy + PartialOrd + Add<Output = T> + Div<Output = T> + From<u8> + Default,
{
    /// Create a new moving median filter with a fixed-size buffer of size N.
    pub fn new() -> Self {
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
    pub fn add_value(&mut self, value: T) {
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
    /// The median is the middle value when the values are sorted in ascending order.
    /// If the number of values is even, the median is the average of the two middle values.
    /// If the number of values is odd, the median is the middle value.
    pub fn median(&self) -> T {
        // If no values have been added, return 0.0
        if self.count == 0 {
            return T::from(0);
        }

        // Create a copy of the buffer and sort it using bubble sort
        let mut sorted_buffer = self.buffer;
        for i in 0..self.count {
            for j in 0..self.count - i - 1 {
                if sorted_buffer[j] > sorted_buffer[j + 1] {
                    sorted_buffer.swap(j, j + 1);
                }
            }
        }

        // Find the median
        if self.count % 2 == 0 {
            // Even number of elements, take the average of the two middle elements
            (sorted_buffer[self.count / 2 - 1] + sorted_buffer[self.count / 2]) / T::from(2)
        } else {
            // Odd number of elements, take the middle element
            sorted_buffer[self.count / 2]
        }
    }

    /// clear the buffer
    /// The buffer will be filled with default values
    /// The count and index will be set to 0
    pub fn clear(&mut self) {
        self.buffer = [T::default(); N];
        self.count = 0;
        self.index = 0;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn median_is_zero_when_no_values_added() {
        let filter = MovingMedian::<f64, 3>::new();
        assert_eq!(filter.median(), 0.0);
    }

    #[test]
    fn median_is_value_when_one_value_added() {
        let mut filter = MovingMedian::<f64, 3>::new();
        filter.add_value(42.0);
        assert_eq!(filter.median(), 42.0);
    }

    #[test]
    fn median_is_average_of_two_values_when_two_values_added() {
        let mut filter = MovingMedian::<f64, 2>::new();
        filter.add_value(42.0);
        filter.add_value(43.0);
        assert_eq!(filter.median(), 42.5);
    }

    #[test]
    fn median_is_middle_value_when_three_values_added() {
        let mut filter = MovingMedian::<f64, 3>::new();
        filter.add_value(42.0);
        filter.add_value(43.0);
        filter.add_value(41.0);
        assert_eq!(filter.median(), 42.0);
    }

    #[test]
    fn median_is_average_of_two_middle_values_when_four_values_added() {
        let mut filter = MovingMedian::<f64, 4>::new();
        filter.add_value(42.0);
        filter.add_value(43.0);
        filter.add_value(41.0);
        filter.add_value(44.0);
        assert_eq!(filter.median(), 42.5);
    }

    #[test]
    fn median_is_midlle_value_of_n_values_when_more_than_n_values_added() {
        let mut filter = MovingMedian::<f64, 3>::new();
        filter.add_value(42.0); // should be pushed out
        filter.add_value(44.0);
        filter.add_value(43.0); // should be the median
        filter.add_value(41.0);
        assert_eq!(filter.median(), 43.0);
    }

    #[test]
    fn median_is_zero_when_cleared() {
        let mut filter = MovingMedian::<f64, 3>::new();
        filter.add_value(42.0);
        filter.add_value(43.0);
        filter.add_value(41.0);
        filter.clear();
        assert_eq!(filter.median(), 0.0);
    }
}
