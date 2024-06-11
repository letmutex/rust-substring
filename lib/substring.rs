// Not working with multi-byte chars
pub fn substring_v0(text: &str, start: usize, end: usize) -> &str {
    &text[start..end]
}

/// skip and take on [[std::str::Chars]
pub fn substring_v1(text: &str, start: usize, end: usize) -> String {
    text.chars().skip(start).take(end - start).collect()
}

/// Find the byte indices manually
pub fn substring_v2(text: &str, start: usize, end: usize) -> &str {
    let mut curr_byte_idx = 0;
    let mut start_byte_idx = 0;
    let mut end_byte_idx = text.len();
    for (index, ch) in text.chars().enumerate() {
        if index == start {
            start_byte_idx = curr_byte_idx;
        }
        if index == end {
            end_byte_idx = curr_byte_idx;
            break;
        }
        curr_byte_idx += ch.len_utf8();
    }
    &text[start_byte_idx..end_byte_idx]
}

/// Find the byte indices using [str::char_indices]
pub fn substring_v3(text: &str, start: usize, end: usize) -> &str {
    let start_byte_idx = text.char_indices().nth(start).map(|(i, _)| i).unwrap_or(0);
    let end_byte_idx = text
        .char_indices()
        .nth(end)
        .map(|(i, _)| i)
        .unwrap_or(text.len());
    &text[start_byte_idx..end_byte_idx]
}

/// Find the byte indices using single [str::char_indices]
pub fn substring_v4(text: &str, start: usize, end: usize) -> &str {
    if start == end {
        return "";
    }
    let mut iter = text.char_indices();
    let start_byte_idx = iter.nth(start).map(|(i, _)| i).unwrap_or(0);
    let end_byte_idx = iter
        .nth(end - start - 1)
        .map(|(i, _)| i)
        .unwrap_or(text.len());
    &text[start_byte_idx..end_byte_idx]
}

/// Left-right char skipping on [std::str::Chars]
pub fn substring_v5(text: &str, start: usize, end: usize) -> &str {
    let mut iter = text.chars();
    let mut left = 0;
    let mut right = iter.clone().count();
    loop {
        if left < start {
            iter.next();
            left += 1;
        }
        if right > end {
            iter.next_back();
            right -= 1;
        }
        if left == start && right <= end {
            break;
        }
    }
    iter.as_str()
}

#[cfg(test)]
mod test {
    use crate::{substring_v1, substring_v2, substring_v3, substring_v4, substring_v5};

    struct Case {
        text: &'static str,
        start: usize,
        end: usize,
        expected: &'static str,
    }

    #[test]
    fn test_substring_v1() {
        verify_cases_with_fn_string(substring_v1);
    }

    #[test]
    fn test_substring_v2() {
        verify_cases_with_fn(substring_v2);
    }

    #[test]
    fn test_substring_v3() {
        verify_cases_with_fn(substring_v3);
    }

    #[test]
    fn test_substring_v4() {
        verify_cases_with_fn(substring_v4);
    }

    #[test]
    fn test_substring_v5() {
        verify_cases_with_fn(substring_v5);
    }

    fn verify_cases_with_fn_string(func: impl Fn(&str, usize, usize) -> String) {
        for case in cases() {
            assert_eq!(case.expected, func(case.text, case.start, case.end));
        }
    }

    fn verify_cases_with_fn(func: impl Fn(&str, usize, usize) -> &str) {
        for case in cases() {
            assert_eq!(case.expected, func(case.text, case.start, case.end));
        }
    }

    fn cases() -> Vec<Case> {
        vec![
            Case {
                text: "Hello world",
                start: 0,
                end: 11,
                expected: "Hello world",
            },
            Case {
                text: "Hello world",
                start: 0,
                end: 20,
                expected: "Hello world",
            },
            Case {
                text: "Hello world",
                start: 1,
                end: 5,
                expected: "ello",
            },
            Case {
                text: "Hello world",
                start: 0,
                end: 0,
                expected: "",
            },
            Case {
                text: "Hello✨, 🎈 this 🎉 is a text.",
                start: 2,
                end: 20,
                expected: "llo✨, 🎈 this 🎉 is ",
            },
            Case {
                text: "こんにちは世界！",
                start: 3,
                end: 7,
                expected: "ちは世界",
            },
            Case {
                text: "你好，世界！",
                start: 3,
                end: 5,
                expected: "世界",
            },
            Case {
                text: "This is 👩‍👨‍👦, A ZWJ emoji",
                start: 8,
                end: 13,
                expected: "👩‍👨‍👦", // 👩 + U+200D + 👨 + U+200D + 👦
            },
        ]
    }
}
