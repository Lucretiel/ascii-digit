#![no_std]

/*!
`AsciiDigit` is a simple type that contains an ascii digit. It's designed such
that strings and byte slices can be zero-copy converted into `&[AsciiDigit]`,
so long as they only contain ascii digits.

This type is primarily intended to be used as a parsing aid, where parsers can
convert substrings into ascii byte slices and then more readily use them or
convert them into real numbers without having to repeat type checks or
reimplement ascii-to-number conversions. It's a low-level primitive that mostly
serves to enclose the various checked unsafe operations that convert between
unchecked `[u8]` and `str` to checked `[AsciiDigit]`.

# Example

```
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
*/

/**
`repr(u8)` type containing an ascii digit. See [crate docs][crate] for more
details.
*/
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
#[repr(u8)]
pub enum AsciiDigit {
    #[default]
    Zero = b'0',
    One,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
}

impl AsciiDigit {
    /**
    Get the numeric value of this digit.

    # Example

    ```
    use ascii_digit::AsciiDigit;

    assert_eq!(AsciiDigit::Three.value(), 3);
    assert_eq!(AsciiDigit::Five.value(), 5);
    ```
    */
    #[inline]
    #[must_use]
    pub const fn value(self) -> u8 {
        (self as u8) - b'0'
    }

    /**
    Convert this digit into its equivalent character.

    # Example

    ```
    use ascii_digit::AsciiDigit;

    assert_eq!(AsciiDigit::Three.as_char(), '3');
    assert_eq!(AsciiDigit::Five.as_char(), '5');
    ```
    */
    #[inline]
    #[must_use]
    pub const fn as_char(self) -> char {
        self as u8 as char
    }

    /**
    Convert an ascii byte into an ascii digit without checking that it's a
    valid ascii digit.

    # Safety

    The byte must be an ascii digit; that is, it must be in the range `b'0'`
    to `b'9'`, inclusive.

    # Example

    ```
    use ascii_digit::AsciiDigit;

    unsafe {
        assert_eq!(AsciiDigit::from_byte_unchecked(b'3'), AsciiDigit::Three);
        assert_eq!(AsciiDigit::from_byte_unchecked(b'5'), AsciiDigit::Five);
    }
    ```
    */
    #[inline]
    #[must_use]
    pub const unsafe fn from_byte_unchecked(byte: u8) -> Self {
        unsafe { *(&byte as *const u8 as *const Self) }
    }

    /**
    Convert a character into an ascii digit without checking that it's a valid
    ascii digit. Note that this is only valid for the code points `0x30..0x3A`.

    # Safety

    The character must be an ascii digit: `'0'` to `'9'`.

    # Example

    ```
    use ascii_digit::AsciiDigit;

    unsafe {
        assert_eq!(AsciiDigit::from_char_unchecked('3'), AsciiDigit::Three);
        assert_eq!(AsciiDigit::from_char_unchecked('5'), AsciiDigit::Five);
    }
    ```
    */
    #[inline]
    #[must_use]
    pub const unsafe fn from_char_unchecked(c: char) -> Self {
        unsafe { Self::from_byte_unchecked(c as u8) }
    }

    /**
    Convert an ascii byte into an ascii digit. Returns none if the byte isn't
    in the range `b'0'` to `b'9'`.

    # Example

    ```
    use ascii_digit::AsciiDigit;

    assert_eq!(AsciiDigit::from_byte(b'3'), Some(AsciiDigit::Three));
    assert_eq!(AsciiDigit::from_byte(b'5'), Some(AsciiDigit::Five));
    assert_eq!(AsciiDigit::from_byte(0), None);
    assert_eq!(AsciiDigit::from_byte(0xFF), None);
    ```
    */
    #[inline]
    #[must_use]
    pub const fn from_byte(byte: u8) -> Option<Self> {
        match byte {
            b'0' => Some(Self::Zero),
            b'1' => Some(Self::One),
            b'2' => Some(Self::Two),
            b'3' => Some(Self::Three),
            b'4' => Some(Self::Four),
            b'5' => Some(Self::Five),
            b'6' => Some(Self::Six),
            b'7' => Some(Self::Seven),
            b'8' => Some(Self::Eight),
            b'9' => Some(Self::Nine),
            _ => None,
        }
    }

    /**
    Convert a character into an ascii digit. Returns none if the character isn't
    in the range `'0'` to `'9'`.

    # Example

    ```
    use ascii_digit::AsciiDigit;

    assert_eq!(AsciiDigit::from_char('3'), Some(AsciiDigit::Three));
    assert_eq!(AsciiDigit::from_char('5'), Some(AsciiDigit::Five));
    assert_eq!(AsciiDigit::from_char('\0'), None);
    assert_eq!(AsciiDigit::from_char('の'), None);
    assert_eq!(AsciiDigit::from_char('⑨'), None);

    ```
    */
    #[inline]
    #[must_use]
    pub const fn from_char(c: char) -> Option<Self> {
        match c {
            '0' => Some(Self::Zero),
            '1' => Some(Self::One),
            '2' => Some(Self::Two),
            '3' => Some(Self::Three),
            '4' => Some(Self::Four),
            '5' => Some(Self::Five),
            '6' => Some(Self::Six),
            '7' => Some(Self::Seven),
            '8' => Some(Self::Eight),
            '9' => Some(Self::Nine),
            _ => None,
        }
    }

    /**
    Convert a slice of bytes into a slice of ascii digits without checking
    that they're all ascii digits.

    # Safety

    *All* bytes in the slice *must* be ascii digits (in the range `b'0'` to
    `b'9'`)

    # Example

    ```
    use ascii_digit::AsciiDigit;

    unsafe {
        assert_eq!(
            AsciiDigit::from_bytes_unchecked(b"123"),
            &[AsciiDigit::One, AsciiDigit::Two, AsciiDigit::Three],
        );
    }
    ```
    */
    #[inline]
    #[must_use]
    pub const unsafe fn from_bytes_unchecked(bytes: &[u8]) -> &[Self] {
        unsafe { &*(bytes as *const [u8] as *const [Self]) }
    }

    /**
    Convert a string into a slice of ascii digits without checking that
    they're all ascii digits.

    # Safety

    The string must *only* contain ascii digits (in the range `'0'` to `'9'`)

    # Example

    ```
    use ascii_digit::AsciiDigit;

    unsafe {
        assert_eq!(
            AsciiDigit::from_str_unchecked("123"),
            &[AsciiDigit::One, AsciiDigit::Two, AsciiDigit::Three],
        );
    }
    ```
    */
    #[inline]
    #[must_use]
    pub const unsafe fn from_str_unchecked(s: &str) -> &[Self] {
        unsafe { Self::from_bytes_unchecked(s.as_bytes()) }
    }

    /**
    Convert a slice of bytes into a slice of ascii digits. Returns `None` if
    any of the bytes aren't valid ascii digits. See
    [`bytes_split_leading_digits`][Self::bytes_split_leading_digits] for a
    version that returns the prefix of the slice that are digits.

    # Example

    ```
    use ascii_digit::AsciiDigit;

    assert_eq!(
        AsciiDigit::from_bytes(b"123").unwrap(),
        &[AsciiDigit::One, AsciiDigit::Two, AsciiDigit::Three],
    );
    assert_eq!(
        AsciiDigit::from_bytes(b"abc"),
        None,
    );
    assert_eq!(
        AsciiDigit::from_bytes(b"123abc"),
        None,
    );
    ```
    */
    #[must_use]
    pub const fn from_bytes(bytes: &[u8]) -> Option<&[Self]> {
        match Self::bytes_split_leading_digits(bytes) {
            (digits, &[]) => Some(digits),
            _ => None,
        }
    }

    /**
    Convert a string into a slice of ascii digits. Returns `None` if the string
    contains anything other than ascii digits. See
    [`str_split_leading_digits`][Self::str_split_leading_digits] for a
    version that returns the prefix of the string that are digits.

    # Example

    ```
    use ascii_digit::AsciiDigit;

    assert_eq!(
        AsciiDigit::from_str("123").unwrap(),
        &[AsciiDigit::One, AsciiDigit::Two, AsciiDigit::Three],
    );
    assert_eq!(
        AsciiDigit::from_str("abc"),
        None,
    );
    assert_eq!(
        AsciiDigit::from_str("123abc"),
        None,
    );
    ```
    */
    #[inline]
    #[must_use]
    pub const fn from_str(s: &str) -> Option<&[Self]> {
        Self::from_bytes(s.as_bytes())
    }

    /**
    Return the prefix of the byte slice that is ascii digits, along with the
    suffix containing the rest of the slice.

    # Example

    ```
    use ascii_digit::AsciiDigit;

    let (head, tail) = AsciiDigit::bytes_split_leading_digits(b"123");
    assert_eq!(head, &[AsciiDigit::One, AsciiDigit::Two, AsciiDigit::Three]);
    assert_eq!(tail, &[]);

    let (head, tail) = AsciiDigit::bytes_split_leading_digits(b"abc");
    assert_eq!(head, &[]);
    assert_eq!(tail, b"abc");

    let (head, tail) = AsciiDigit::bytes_split_leading_digits(b"123abc");
    assert_eq!(head, &[AsciiDigit::One, AsciiDigit::Two, AsciiDigit::Three]);
    assert_eq!(tail, b"abc");
    ```
    */
    #[must_use]
    pub const fn bytes_split_leading_digits(bytes: &[u8]) -> (&[Self], &[u8]) {
        let mut iter = bytes;

        while let &[b, ref tail @ ..] = iter {
            iter = if !b.is_ascii_digit() {
                let tail_len = iter.len();
                let head_len = bytes.len() - tail_len;
                let head = unsafe { bytes.split_at_unchecked(head_len).0 };
                let head = unsafe { Self::from_bytes_unchecked(head) };
                return (head, iter);
            } else {
                tail
            }
        }

        (unsafe { Self::from_bytes_unchecked(bytes) }, &[])
    }

    /**
    Return the prefix of the string that is ascii digits, along with the
    suffix containing the rest of the string.

    # Example

    ```
    use ascii_digit::AsciiDigit;

    let (head, tail) = AsciiDigit::str_split_leading_digits("123");
    assert_eq!(head, &[AsciiDigit::One, AsciiDigit::Two, AsciiDigit::Three]);
    assert_eq!(tail, "");

    let (head, tail) = AsciiDigit::str_split_leading_digits("abc");
    assert_eq!(head, &[]);
    assert_eq!(tail, "abc");

    let (head, tail) = AsciiDigit::str_split_leading_digits("123abc");
    assert_eq!(head, &[AsciiDigit::One, AsciiDigit::Two, AsciiDigit::Three]);
    assert_eq!(tail, "abc");
    ```
    */
    #[inline]
    #[must_use]
    pub const fn str_split_leading_digits(s: &str) -> (&[Self], &str) {
        let (digits, tail) = Self::bytes_split_leading_digits(s.as_bytes());

        // SAFETY: we only trimmed ascii bytes off of the front of the string,
        // so whatever's left in the tail must also be a valid utf8 string
        let tail = unsafe { str::from_utf8_unchecked(tail) };
        (digits, tail)
    }
}

impl From<AsciiDigit> for char {
    #[inline]
    fn from(value: AsciiDigit) -> char {
        value.as_char()
    }
}

impl TryFrom<char> for AsciiDigit {
    type Error = NotDigit;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        AsciiDigit::from_char(value).ok_or(NotDigit)
    }
}

/// Helper trait for converting `str` and `[u8]` into `[AsciiDigit]`.
pub trait StrExt {
    /**
    Convert this string into a `&[AsciiDigit]` without verifying that it only
    contains ascii digits.

    # Safety

    `self` must only contain ascii digits. Implementors are allowed to assume
    the caller has performed this check already and may use internal unchecked
    operations.
    */
    #[must_use]
    unsafe fn as_ascii_digits_unchecked(&self) -> &[AsciiDigit];

    /**
    Convert this string into a `&[AsciiDigit]`. Returns `None` if `self`
    contains anything other than all ascii digits.
    */
    #[must_use]
    fn as_ascii_digits(&self) -> Option<&[AsciiDigit]>;

    /**
    Return the prefix of the string that is ascii digits, along with the suffix
    containing the rest of the string.
    */
    #[must_use]
    fn split_leading_ascii_digits(&self) -> (&[AsciiDigit], &Self);
}

impl StrExt for [u8] {
    #[inline]
    unsafe fn as_ascii_digits_unchecked(&self) -> &[AsciiDigit] {
        unsafe { AsciiDigit::from_bytes_unchecked(self) }
    }

    #[inline]
    fn as_ascii_digits(&self) -> Option<&[AsciiDigit]> {
        AsciiDigit::from_bytes(self)
    }

    #[inline]
    fn split_leading_ascii_digits(&self) -> (&[AsciiDigit], &Self) {
        AsciiDigit::bytes_split_leading_digits(self)
    }
}

impl StrExt for str {
    #[inline]
    unsafe fn as_ascii_digits_unchecked(&self) -> &[AsciiDigit] {
        unsafe { AsciiDigit::from_str_unchecked(self) }
    }

    #[inline]
    fn as_ascii_digits(&self) -> Option<&[AsciiDigit]> {
        AsciiDigit::from_str(self)
    }

    #[inline]
    fn split_leading_ascii_digits(&self) -> (&[AsciiDigit], &Self) {
        AsciiDigit::str_split_leading_digits(self)
    }
}

/**
Error type indicating that a `TryFrom` conversion failed because the input
wasn't a digit
*/
#[derive(Debug, Clone, Copy)]
pub struct NotDigit;

impl ::core::fmt::Display for NotDigit {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "character wasn't an ascii digit")
    }
}

impl ::core::error::Error for NotDigit {}
