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
//! ```Rust
//! use moving_median::MovingMedian;
//! 
//! let mut filter = MovingMedian::<3>::new();
//! filter.add_value(42.0);
//! filter.add_value(43.0);
//! filter.add_value(41.0);
//! 
//! assert_eq!(filter.median(), 42.0);
//! ```

#![no_std]

pub struct MovingMedian<const N: usize> {
    buffer: [f64; N], // Fixed-size buffer to hold the measurements
    index: usize,     // Current index in the buffer
    count: usize,     // Number of values added (up to N)
}

impl<const N: usize> MovingMedian<N> {
    /// Create a new moving median filter with a fixed-size buffer of size N.
    pub fn new() -> Self {
        Self {
            buffer: [0.0; N],
            index: 0,
            count: 0,
        }
    }

    /// Add a new measurement to the buffer.
    /// If the buffer is full, the oldest value will be replaced.
    /// The buffer will always contain the last N measurements.
    /// The count will be incremented up to N.
    pub fn add_value(&mut self, value: f64) {
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
    pub fn median(&self) -> f64 {
        // If no values have been added, return 0.0
        if self.count == 0 {
            return 0.0;
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
            (sorted_buffer[self.count / 2 - 1] + sorted_buffer[self.count / 2]) / 2.0
        } else {
            // Odd number of elements, take the middle element
            sorted_buffer[self.count / 2]
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn median_is_zero_when_no_values_added() {
        let filter = MovingMedian::<3>::new();
        assert_eq!(filter.median(), 0.0);   
    }

    #[test]
    fn median_is_value_when_one_value_added() {
        let mut filter = MovingMedian::<3>::new();
        filter.add_value(42.0);
        assert_eq!(filter.median(), 42.0);   
    }

    #[test]
    fn median_is_average_of_two_values_when_two_values_added() {
        let mut filter = MovingMedian::<2>::new();
        filter.add_value(42.0);
        filter.add_value(43.0);
        assert_eq!(filter.median(), 42.5);   
    }

    #[test]
    fn median_is_middle_value_when_three_values_added() {
        let mut filter = MovingMedian::<3>::new();
        filter.add_value(42.0);
        filter.add_value(43.0);
        filter.add_value(41.0);
        assert_eq!(filter.median(), 42.0);   
    }

    #[test]
    fn median_is_average_of_two_middle_values_when_four_values_added() {
        let mut filter = MovingMedian::<4>::new();
        filter.add_value(42.0);
        filter.add_value(43.0);
        filter.add_value(41.0);
        filter.add_value(44.0);
        assert_eq!(filter.median(), 42.5);   
    }

    #[test]
    fn median_is_midlle_value_of_n_values_when_more_than_n_values_added()
    {
        let mut filter = MovingMedian::<3>::new();
        filter.add_value(42.0); // should be pushed out
        filter.add_value(44.0);
        filter.add_value(43.0); // should be the median
        filter.add_value(41.0);
        assert_eq!(filter.median(), 43.0);   
    } 
}
