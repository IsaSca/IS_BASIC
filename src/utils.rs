pub(crate) fn extract_digits(s: &str) -> (&str, &str) {
    let digits_end = s
        .char_indices()
        .find_map(|(idx,c)| if c.is_ascii_digit() {None} else {Some(idx)})
        .unwrap_or_else(||s.len());
    let digits = &s[..digits_end];
    let remainder = &s[digits_end..];
    (remainder, digits)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn extract_one_digit() {
        assert_eq!(extract_digits("1+2"), ("+2", "1"));
    }

    #[test]
    fn no_extract_empty_in() {
        assert_eq!(extract_digits(""), ("",""));
    }

    #[test]
    fn extract_no_r() {
        assert_eq!(extract_digits("100"), ("","100"))
    }
}