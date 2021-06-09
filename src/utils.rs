const WHITESPACE: &[char] = &[' ', '\n'];

pub(crate) fn take_while(accept: impl Fn(char) -> bool, s: &str) -> (&str, &str) {
    let extracted_end = s
        .char_indices()
        .find_map(|(idx, c)| if accept(c) {
            None
        } else {
            Some(idx)
        })
        .unwrap_or_else(||s.len());

    let extracted = &s[..extracted_end];
    let remainder = &s[extracted_end..];
    (remainder, extracted)
}

pub(crate) fn take_while1(
    accept: impl Fn(char) -> bool,
    s: &str,
    error_msg: String,
) -> Result<(&str, &str), String> {
    let (remainder, extracted) = take_while(accept, s);
    if extracted.is_empty() {
        Err(error_msg)
    } else {
        Ok((remainder, extracted))
    }
}

pub(crate) fn extract_digits(s: &str) -> Result<(&str, &str), String> {
    take_while1(|c| c.is_ascii_digit(), s, "expected digits".to_string())
}

pub(crate) fn extract_operator(s: &str) -> (&str, &str) {
    match &s[0..1] {
        "+" | "-" | "*" | "/" => {}
        _ => panic!("Bad Operator"),
    }

    (&s[1..], &s[0..1])
}

pub(crate) fn extract_whitespace(s: &str) -> (&str, &str) {
    take_while(|c| WHITESPACE.contains(&c), s)
}

pub(crate) fn extract_whitespace1(s: &str) -> Result<(&str, &str), String> {
    take_while1(
        |c| WHITESPACE.contains(&c), 
        s, 
        "expected whitespace".to_string())
}


pub(crate) fn tag<'a, 'b>(starting_text:&'a str, s: &'b str) -> Result<&'b str, String> {
    if s.starts_with(starting_text) {
        Ok(&s[starting_text.len()..])
    } else {
        Err(format!("Expected {} at the disco", starting_text))
    }
}

pub(crate) fn extract_ident(s: &str) -> Result<(&str, &str), String> {
    let input_starts_with_alpha = s
        .chars()
        .next()
        .map(|c| c.is_ascii_alphabetic())
        .unwrap_or(false);

    if input_starts_with_alpha {
        Ok(take_while(|c| c.is_ascii_alphanumeric(), s))
    } else {
        Err("expected identifier".to_string())
    }

}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn extract_one_digit() {
        assert_eq!(extract_digits("1+2"), Ok(("+2", "1")));
    }

    #[test]
    fn no_extract_digit_if_invalid() {
        assert_eq!(extract_digits("abcd"), Err("expected digits".to_string()));
    }

    #[test]
    fn extract_no_r() {
        assert_eq!(extract_digits("100"), Ok(("","100")));
    }

    #[test]
    fn extract_pl() {
        assert_eq!(extract_operator("+2"), ("2", "+"));
    }
    #[test]
    fn extract_min() {
        assert_eq!(extract_operator("-2"), ("2", "-"));
    }
    #[test]
    fn extract_mul() {
        assert_eq!(extract_operator("*2"), ("2", "*"));
    }
    #[test]
    fn extract_div() {
        assert_eq!(extract_operator("/2"), ("2", "/"));
    }

    #[test]
    fn extract_white() {
        assert_eq!(extract_whitespace(" 1"), ("1", " "));
    }

    #[test]
    fn do_not_extract_white_when_input_no_start_with() {
        assert_eq!(
            extract_whitespace1("blah"),
            Err("expected whitespace".to_string()),
        );
    }

    #[test]
    fn extract_alpha_ident() {
        assert_eq!(extract_ident("abcd1 stop"), Ok((" stop", "abcd1")));
    }

    #[test]
    fn not_extract_ident_as_starts_with_num() {
        assert_eq!(extract_ident("123abc"), Err("expected identifier".to_string()),
        );
    }

    #[test]
    fn tag_word() {
        assert_eq!(tag("let", "let a"), Ok(" a"));
    }
}