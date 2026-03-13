//! U.S. Employer Identification Number (EIN) parsing and validation.
//!
//! # Example
//!
//! ```
//! use ein::Ein;
//!
//! let ein: Ein = "12-3456789".parse().unwrap();
//! assert_eq!(ein.to_string(), "12-3456789");
//!
//! // Also accepts format without dash
//! let ein: Ein = "123456789".parse().unwrap();
//! assert_eq!(ein.to_string(), "12-3456789");
//! ```

use core::fmt;
use core::str::FromStr;

use regex::Regex;

#[cfg(feature = "serde")]
use serde::{Deserialize, Deserializer, Serialize, Serializer};

/// Matches EIN in format XX-XXXXXXX or XXXXXXXXX (9 consecutive digits).
static EIN_PATTERN: &str = r"^(\d{2})-(\d{7})$|^(\d{9})$";

/// IRS prefixes that have never been assigned and are therefore invalid.
const INVALID_PREFIXES: [u8; 17] = [
    0, 7, 8, 9, 17, 18, 19, 28, 29, 49, 69, 70, 78, 79, 89, 96, 97,
];

/// A validated U.S. Employer Identification Number.
///
/// # Format
///
/// EINs are 9-digit numbers formatted as `XX-XXXXXXX`, where the first two
/// digits are the IRS-assigned prefix.
///
/// # Validation
///
/// The prefix must be a valid IRS-assigned prefix. Invalid prefixes include:
/// 00, 07, 08, 09, 17, 18, 19, 28, 29, 49, 69, 70, 78, 79, 89, 96, 97.
#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct Ein {
    prefix: u8,
    serial: u32,
}

/// Errors that can occur when parsing an EIN.
#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum ParseError {
    /// The input string does not match the expected EIN format.
    #[error("invalid format '{0}': expected XX-XXXXXXX or XXXXXXXXX")]
    InvalidFormat(String),
    /// The prefix is not a valid IRS-assigned prefix.
    #[error("invalid prefix: {0:02} (not an IRS-assigned prefix)")]
    InvalidPrefix(u8),
}

impl Ein {
    /// Creates a new EIN from its components.
    pub fn new(prefix: u8, serial: u32) -> Result<Self, ParseError> {
        Self::validate(prefix, serial)?;
        Ok(Self { prefix, serial })
    }

    fn validate(prefix: u8, serial: u32) -> Result<(), ParseError> {
        if prefix > 99 || INVALID_PREFIXES.contains(&prefix) {
            return Err(ParseError::InvalidPrefix(prefix));
        }
        // serial is at most 9999999 (7 digits) which fits in u32;
        // values above that can only come from Ein::new, not from parsing
        if serial > 9_999_999 {
            // Not reachable from FromStr, but guard the constructor
            return Err(ParseError::InvalidFormat(format!("{prefix:02}{serial:07}")));
        }
        Ok(())
    }

    /// Returns the prefix (first 2 digits).
    pub fn prefix(&self) -> u8 {
        self.prefix
    }

    /// Returns the serial number (last 7 digits).
    pub fn serial(&self) -> u32 {
        self.serial
    }
}

impl FromStr for Ein {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let re = Regex::new(EIN_PATTERN)
            .expect("EIN_PATTERN is a valid regex: two alternates for dashed and undashed formats");
        let caps = re
            .captures(s)
            .ok_or_else(|| ParseError::InvalidFormat(s.to_owned()))?;

        let (prefix_str, serial_str) = if let (Some(p), Some(s)) = (caps.get(1), caps.get(2)) {
            (p.as_str(), s.as_str())
        } else if let Some(full) = caps.get(3) {
            let full = full.as_str();
            (&full[0..2], &full[2..9])
        } else {
            return Err(ParseError::InvalidFormat(s.to_owned()));
        };

        let prefix: u8 = prefix_str.parse().expect(
            "prefix is exactly two ASCII digits as enforced by EIN_PATTERN; parse::<u8> cannot fail",
        );
        let serial: u32 = serial_str.parse().expect(
            "serial is exactly seven ASCII digits as enforced by EIN_PATTERN; parse::<u32> cannot fail",
        );

        Self::new(prefix, serial)
    }
}

impl fmt::Display for Ein {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:02}-{:07}", self.prefix, self.serial)
    }
}

impl fmt::Debug for Ein {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Ein({:02}-XXXXXXX)", self.prefix)
    }
}

#[cfg(feature = "serde")]
impl Serialize for Ein {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_str(&self.to_string())
    }
}

#[cfg(feature = "serde")]
impl<'de> Deserialize<'de> for Ein {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let s = String::deserialize(deserializer)?;
        s.parse().map_err(serde::de::Error::custom)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // --- Valid parsing ---

    #[test]
    fn valid_ein_with_dash() {
        let ein: Ein = "12-3456789".parse().unwrap();
        assert_eq!(ein.prefix(), 12);
        assert_eq!(ein.serial(), 3_456_789);
        assert_eq!(ein.to_string(), "12-3456789");
    }

    #[test]
    fn valid_ein_no_dash() {
        let ein: Ein = "123456789".parse().unwrap();
        assert_eq!(ein.to_string(), "12-3456789");
    }

    #[test]
    fn valid_ein_from_components() {
        let ein = Ein::new(12, 3_456_789).unwrap();
        assert_eq!(ein.prefix(), 12);
        assert_eq!(ein.serial(), 3_456_789);
    }

    #[test]
    fn valid_ein_zero_serial() {
        let ein = Ein::new(10, 0).unwrap();
        assert_eq!(ein.to_string(), "10-0000000");
    }

    #[test]
    fn valid_ein_max_serial() {
        let ein = Ein::new(10, 9_999_999).unwrap();
        assert_eq!(ein.to_string(), "10-9999999");
    }

    // --- Valid prefixes ---

    #[test]
    fn valid_common_prefixes() {
        for prefix in [
            1, 2, 10, 11, 12, 20, 30, 40, 50, 60, 65, 71, 80, 90, 95, 98, 99,
        ] {
            assert!(
                Ein::new(prefix, 0).is_ok(),
                "prefix {prefix} should be valid"
            );
        }
    }

    // --- Invalid prefixes ---

    #[test]
    fn invalid_prefix_00() {
        assert!(matches!(
            "00-1234567".parse::<Ein>(),
            Err(ParseError::InvalidPrefix(0))
        ));
    }

    #[test]
    fn invalid_prefix_07() {
        assert!(matches!(Ein::new(7, 0), Err(ParseError::InvalidPrefix(7))));
    }

    #[test]
    fn invalid_prefix_08() {
        assert!(matches!(Ein::new(8, 0), Err(ParseError::InvalidPrefix(8))));
    }

    #[test]
    fn invalid_prefix_09() {
        assert!(matches!(Ein::new(9, 0), Err(ParseError::InvalidPrefix(9))));
    }

    #[test]
    fn invalid_prefix_17() {
        assert!(matches!(
            Ein::new(17, 0),
            Err(ParseError::InvalidPrefix(17))
        ));
    }

    #[test]
    fn invalid_prefix_18() {
        assert!(matches!(
            Ein::new(18, 0),
            Err(ParseError::InvalidPrefix(18))
        ));
    }

    #[test]
    fn invalid_prefix_19() {
        assert!(matches!(
            Ein::new(19, 0),
            Err(ParseError::InvalidPrefix(19))
        ));
    }

    #[test]
    fn invalid_prefix_28() {
        assert!(matches!(
            Ein::new(28, 0),
            Err(ParseError::InvalidPrefix(28))
        ));
    }

    #[test]
    fn invalid_prefix_29() {
        assert!(matches!(
            Ein::new(29, 0),
            Err(ParseError::InvalidPrefix(29))
        ));
    }

    #[test]
    fn invalid_prefix_49() {
        assert!(matches!(
            Ein::new(49, 0),
            Err(ParseError::InvalidPrefix(49))
        ));
    }

    #[test]
    fn invalid_prefix_69() {
        assert!(matches!(
            Ein::new(69, 0),
            Err(ParseError::InvalidPrefix(69))
        ));
    }

    #[test]
    fn invalid_prefix_70() {
        assert!(matches!(
            Ein::new(70, 0),
            Err(ParseError::InvalidPrefix(70))
        ));
    }

    #[test]
    fn invalid_prefix_78() {
        assert!(matches!(
            Ein::new(78, 0),
            Err(ParseError::InvalidPrefix(78))
        ));
    }

    #[test]
    fn invalid_prefix_79() {
        assert!(matches!(
            Ein::new(79, 0),
            Err(ParseError::InvalidPrefix(79))
        ));
    }

    #[test]
    fn invalid_prefix_89() {
        assert!(matches!(
            Ein::new(89, 0),
            Err(ParseError::InvalidPrefix(89))
        ));
    }

    #[test]
    fn invalid_prefix_96() {
        assert!(matches!(
            Ein::new(96, 0),
            Err(ParseError::InvalidPrefix(96))
        ));
    }

    #[test]
    fn invalid_prefix_97() {
        assert!(matches!(
            Ein::new(97, 0),
            Err(ParseError::InvalidPrefix(97))
        ));
    }

    // --- Invalid formats ---

    #[test]
    fn invalid_format_letters() {
        assert!(matches!(
            "1a-3456789".parse::<Ein>(),
            Err(ParseError::InvalidFormat(_))
        ));
    }

    #[test]
    fn invalid_format_too_short() {
        assert!(matches!(
            "12-345678".parse::<Ein>(),
            Err(ParseError::InvalidFormat(_))
        ));
    }

    #[test]
    fn invalid_format_too_long() {
        assert!(matches!(
            "12-34567890".parse::<Ein>(),
            Err(ParseError::InvalidFormat(_))
        ));
    }

    #[test]
    fn invalid_format_wrong_separator() {
        assert!(matches!(
            "12+3456789".parse::<Ein>(),
            Err(ParseError::InvalidFormat(_))
        ));
    }

    #[test]
    fn invalid_serial_out_of_bounds() {
        assert!(Ein::new(10, 10_000_000).is_err());
    }

    // --- Display / Debug ---

    #[test]
    fn display_format() {
        let ein: Ein = "12-3456789".parse().unwrap();
        assert_eq!(ein.to_string(), "12-3456789");
    }

    #[test]
    fn debug_masks_serial() {
        let ein: Ein = "12-3456789".parse().unwrap();
        assert_eq!(format!("{ein:?}"), "Ein(12-XXXXXXX)");
    }

    #[test]
    fn debug_pads_prefix() {
        let ein = Ein::new(1, 0).unwrap();
        assert_eq!(format!("{ein:?}"), "Ein(01-XXXXXXX)");
    }

    // --- Copy ---

    #[test]
    fn ein_is_copy() {
        let a = Ein::new(12, 3_456_789).unwrap();
        let b = a; // copy
        assert_eq!(a, b);
    }
}

#[cfg(all(test, feature = "serde"))]
mod serde_tests {
    use super::*;

    #[test]
    fn serialize_to_string() {
        let ein: Ein = "12-3456789".parse().unwrap();
        let json = serde_json::to_string(&ein).unwrap();
        assert_eq!(json, "\"12-3456789\"");
    }

    #[test]
    fn deserialize_with_dash() {
        let ein: Ein = serde_json::from_str("\"12-3456789\"").unwrap();
        assert_eq!(ein.prefix(), 12);
        assert_eq!(ein.serial(), 3_456_789);
    }

    #[test]
    fn deserialize_without_dash() {
        let ein: Ein = serde_json::from_str("\"123456789\"").unwrap();
        assert_eq!(ein.to_string(), "12-3456789");
    }

    #[test]
    fn deserialize_invalid_prefix() {
        let result = serde_json::from_str::<Ein>("\"00-1234567\"");
        assert!(result.is_err());
    }

    #[test]
    fn roundtrip() {
        let ein: Ein = "55-1234567".parse().unwrap();
        let json = serde_json::to_string(&ein).unwrap();
        let back: Ein = serde_json::from_str(&json).unwrap();
        assert_eq!(ein, back);
    }
}
