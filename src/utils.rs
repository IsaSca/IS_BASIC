pub(crate) fn extract_digits(s: &str) -> (&str, &str) {
    let digits_end = s
        .char_indices()
        .find_map(|(idx,c)| if c.is_ascii_digit() {None} else {Some(idx)})
        .unwrap_or_else(||s.len());
    let digits = &s[..digits_end];
    let remainder = &s[digits_end..];
    (remainder, digits)
}

pub(crate) fn extract_operator(s: &str) -> (&str, &str) {
    match &s[0..1] {
        "+" | "-" | "*" | "/" => {}
        _ => panic!("Bad Operator"),
    }

    (&s[1..], &s[0..1])
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

    #[test]
    fn extract_pl() {
        assert_eq!(extract_operator("+2"), ("2", "+"))
    }
    #[test]
    fn extract_min() {
        assert_eq!(extract_operator("-2"), ("2", "-"))
    }
    #[test]
    fn extract_mul() {
        assert_eq!(extract_operator("*2"), ("2", "*"))
    }
    #[test]
    fn extract_div() {
        assert_eq!(extract_operator("/2"), ("2", "/"))
    }
}