# ein

U.S. Employer Identification Number (EIN) parsing and validation for Rust.

[![Crates.io](https://img.shields.io/crates/v/ein.svg)](https://crates.io/crates/ein)
[![Documentation](https://docs.rs/ein/badge.svg)](https://docs.rs/ein)
[![CI](https://github.com/gideonsolutions/ein/actions/workflows/ci.yml/badge.svg)](https://github.com/gideonsolutions/ein/actions/workflows/ci.yml)

## Usage

```rust
use ein::Ein;

// Parse from string with dash
let ein: Ein = "12-3456789".parse().unwrap();
assert_eq!(ein.to_string(), "12-3456789");

// Parse from string without dash
let ein: Ein = "123456789".parse().unwrap();
assert_eq!(ein.to_string(), "12-3456789");

// Access components
assert_eq!(ein.prefix(), 12);
assert_eq!(ein.serial(), 3_456_789);

// Create from components
let ein = Ein::new(12, 3_456_789).unwrap();
```

## Validation

Validates that the 2-digit prefix is an IRS-assigned prefix. The following prefixes are **invalid** (never assigned by the IRS):

`00`, `07`, `08`, `09`, `17`, `18`, `19`, `28`, `29`, `49`, `69`, `70`, `78`, `79`, `89`, `96`, `97`

```rust
use ein::{Ein, ParseError};

// Invalid prefix
assert!(matches!(
    "00-1234567".parse::<Ein>(),
    Err(ParseError::InvalidPrefix(0))
));

// Invalid format
assert!(matches!(
    "12+3456789".parse::<Ein>(),
    Err(ParseError::InvalidFormat(_))
));
```

## Privacy

The `Debug` implementation masks the serial number, showing only the prefix:

```rust
let ein: Ein = "12-3456789".parse().unwrap();
assert_eq!(format!("{:?}", ein), "Ein(12-XXXXXXX)");
```

## License

Apache-2.0
