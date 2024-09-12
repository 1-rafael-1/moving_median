[![ci](https://github.com/1-rafael-1/moving_median/actions/workflows/rust.yml/badge.svg)](https://github.com/1-rafael-1/moving_median/actions/workflows/rust.yml)

# moving median

A simple no-std moving median filter implementation with a fixed-size buffer. The buffer is used to store the last N measurements, where N is the size of the buffer. The median is calculated by sorting the values in the buffer and taking the middle value. If the number of values is even, the median is the average of the two middle values. If the number of values is odd, the median is the middle value.

## Example

```Rust
use moving_median::MovingMedian;

let mut filter = MovingMedian::<3>::new();
filter.add_measurement(42.0);
filter.add_measurement(43.0);
filter.add_measurement(41.0);

assert_eq!(filter.median(), 42.0);
```
