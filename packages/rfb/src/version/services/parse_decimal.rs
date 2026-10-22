pub(crate) fn parse_decimal(digits: &[u8]) -> Option<u32> {
    digits.iter().try_fold(0u32, |value, &digit| {
        digit
            .is_ascii_digit()
            .then(|| value * 10 + u32::from(digit - b'0'))
    })
}
