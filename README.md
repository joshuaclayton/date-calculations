# Date Calculations

[![](https://img.shields.io/crates/v/date-calculations.svg)](https://crates.io/crates/date-calculations)
[![](https://docs.rs/date-calculations/badge.svg)](https://docs.rs/date-calculations)

This is a crate supporting relative date calculations for Chrono's [NaiveDate],
most notably only Gregorian dates.

[NaiveDate]: https://docs.rs/chrono/latest/chrono/naive/struct.NaiveDate.html

## Usage

```rust
use chrono::prelude::*;
use date_calculations::*;

let twenty_twenty_one = NaiveDate::from_ymd_opt(2021, 1, 31).unwrap();

assert_eq!(next_year(&twenty_twenty_one).unwrap().year(), 2022);
assert_eq!(next_year(&twenty_twenty_one).unwrap().month(), 1);
assert_eq!(next_year(&twenty_twenty_one).unwrap().day(), 1);

assert_eq!(previous_quarter(&twenty_twenty_one).unwrap().year(), 2020);
assert_eq!(previous_quarter(&twenty_twenty_one).unwrap().month(), 10);
assert_eq!(previous_quarter(&twenty_twenty_one).unwrap().day(), 1);
```

## License

Copyright 2020 Josh Clayton. See the [LICENSE](LICENSE).

