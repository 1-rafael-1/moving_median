[![ci](https://github.com/1-rafael-1/moving_median/actions/workflows/rust.yml/badge.svg)](https://github.com/1-rafael-1/moving_median/actions/workflows/rust.yml)

# moving median

A simple no-std moving median filter implementation with a fixed-size buffer. The buffer is used to store the last N measurements, where N is the size of the buffer. The median is calculated by sorting the values in the buffer and taking the middle value. If the number of values is even, the median is the average of the two middle values. If the number of values is odd, the median is the middle value.

This implementation supports both f32 and f64 types.

## Example

```Rust
use moving_median::MovingMedian;

// f32
let mut filter_f32 = MovingMedian::<f32, 3>::new();
filter_f32.add_value(42.0);
filter_f32.add_value(43.0);
filter_f32.add_value(41.0);
assert_eq!(filter_f32.median(), 42.0);

// f64
let mut filter_f64 = MovingMedian::<f64, 3>::new();
filter_f64.add_value(42.0);
filter_f64.add_value(43.0);
filter_f64.add_value(41.0);
assert_eq!(filter_f64.median(), 42.0);

// clearing existing data
let mut filter_f64 = MovingMedian::<f64, 3>::new();
filter_f64.add_value(42.0);
filter_f64.add_value(43.0);
filter_f64.add_value(41.0);
filter.clear();
assert_eq!(filter.median(), 0.0);
```
