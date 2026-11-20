# ascii-digit

[![CI](https://github.com/Lucretiel/ascii-digit/actions/workflows/ci.yml/badge.svg)](https://github.com/Lucretiel/ascii-digit/actions/workflows/ci.yml)
[![crates.io](https://img.shields.io/crates/v/ascii-digit.svg)](https://crates.io/crates/ascii-digit)
[![docs.rs](https://img.shields.io/docsrs/ascii-digit)](https://docs.rs/ascii-digit)
[![MSRV](https://img.shields.io/crates/msrv/ascii-digit)](https://crates.io/crates/ascii-digit)

A simple rust crate for identifying ascii digits.

<!-- cargo-rdme start -->

`AsciiDigit` is a simple type that contains an ascii digit. It's designed such
that strings and byte slices can be zero-copy converted into `&[AsciiDigit]`,
so long as they only contain ascii digits.

This type is primarily intended to be used as a parsing aid, where parsers can
convert substrings into ascii byte slices and then more readily use them or
convert them into real numbers without having to repeat type checks or
reimplement ascii-to-number conversions. It's a low-level primitive that mostly
serves to enclose the various checked unsafe operations that convert between
unchecked `[u8]` and `str` to checked `[AsciiDigit]`.

## Example

```rust
use ascii_digit::StrExt as _;

fn parse_float(input: &str) -> Option<f64> {
    // int_part is [AsciiDigit]
    let (int_part, input) = input.split_leading_ascii_digits();
    let fraction = match input.strip_prefix('.') {
        None if input.is_empty() => None,
        None => return None,
        Some(suffix) => Some(suffix.as_ascii_digits()?),
    };

    if int_part.is_empty() && fraction.is_none_or(|fraction| fraction.is_empty()) {
        return None
    }

    // AsciiDigit has a `value` method to get its numeric value
    let int_part = int_part.iter().copied().fold(0.0, |accum, digit| {
        let accum = accum * 10.0;
        accum + (digit.value() as f64)
    });

    let frac_part = fraction.unwrap_or(&[]).iter().copied().rev().fold(0.0, |accum, digit| {
        let accum = accum + (digit.value() as f64);
        accum / 10.0
    });

    Some(int_part + frac_part)
}

assert_eq!(parse_float("22.125"), Some(22.125));
assert_eq!(parse_float(".25"), Some(0.25));
assert_eq!(parse_float("10"), Some(10.0));
assert_eq!(parse_float("."), None);
assert_eq!(parse_float("abc"), None);
assert_eq!(parse_float("12abc"), None);
```

<!-- cargo-rdme end -->
