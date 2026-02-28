use std::str::FromStr;
use std::{fmt::Debug, num::ParseIntError};

#[derive(Debug, PartialEq)]
pub enum IndexParseError {
    UnexpectedNumberOfElements(usize),
    NumberParseError(String, ParseIntError),
}

pub fn parse(element: &str) -> Result<(i128, Option<i128>, Option<i128>), IndexParseError> {
    let split: Vec<&str> = element.split("/").collect();

    if split.len() < 1 || split.len() > 3 {
        return Err(IndexParseError::UnexpectedNumberOfElements(split.len()));
    }

    let vertex_index = i128::from_str(split[0]);
    if vertex_index.is_err() {
        return Err(IndexParseError::NumberParseError(
            String::from(split[0]),
            vertex_index.err().unwrap(),
        ));
    }
    let mut texture_index = Option::None;
    if split.len() > 1 && !split[1].is_empty() {
        let parsed = i128::from_str(split[1]);
        if parsed.is_err() {
            return Err(IndexParseError::NumberParseError(
                String::from(split[1]),
                parsed.err().unwrap(),
            ));
        }
        texture_index = Option::Some(parsed.unwrap());
    }
    let mut normal_index = Option::None;
    if split.len() > 2 {
        let parsed = i128::from_str(split[2]);
        if parsed.is_err() {
            return Err(IndexParseError::NumberParseError(
                String::from(split[2]),
                parsed.err().unwrap(),
            ));
        }
        normal_index = Option::Some(parsed.unwrap());
    }

    Ok((vertex_index.unwrap(), texture_index, normal_index))
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::num::IntErrorKind;

    #[test]
    fn test_parse_index_vertex_only() {
        let index = "5";

        let result = parse(index);

        assert_eq!(result, Ok((5, None, None)));
    }

    #[test]
    fn test_parse_index_vertex_and_uv() {
        let index = "5/1";

        let result = parse(index);

        assert_eq!(result, Ok((5, Some(1), None)));
    }

    #[test]
    fn test_parse_index_vertex_and_normal() {
        let index = "5//2";

        let result = parse(index);

        assert_eq!(result, Ok((5, None, Some(2))));
    }

    #[test]
    fn test_parse_index_vertex_uv_and_normal() {
        let index = "5/99/2";

        let result = parse(index);

        assert_eq!(result, Ok((5, Some(99), Some(2))));
    }

    #[test]
    fn test_parse_negative() {
        let index = "-5/-99/-2";

        let result = parse(index);

        assert_eq!(result, Ok((-5, Some(-99), Some(-2))));
    }

    #[test]
    fn test_fails_on_emtpty() {
        let index = "";

        let result = parse(index);

        match result {
            Ok(_) => assert!(false, "Parsing was expected to fail."),
            Err(err) => match err {
                IndexParseError::UnexpectedNumberOfElements(_) => {
                    assert!(false, "Parsing failed due to the wrong reason.")
                }
                IndexParseError::NumberParseError(parsed_line, reason) => {
                    assert_eq!(parsed_line, index);
                    assert_eq!(*reason.kind(), IntErrorKind::Empty);
                }
            },
        }
    }

    #[test]
    fn test_fails_on_to_many_elements() {
        let index = "5/4/3/2";

        let result = parse(index);

        assert_eq!(result, Err(IndexParseError::UnexpectedNumberOfElements(4)));
    }

    #[test]
    fn test_wrong_string_in_vertex_index() {
        let index = "l5/99/2";

        let result = parse(index);

        match result {
            Ok(_) => assert!(false, "Parsing was expected to fail."),
            Err(err) => match err {
                IndexParseError::UnexpectedNumberOfElements(_) => {
                    assert!(false, "Parsing failed due to the wrong reason.")
                }
                IndexParseError::NumberParseError(parsed_line, reason) => {
                    assert_eq!(parsed_line, "l5");
                    assert_eq!(*reason.kind(), IntErrorKind::InvalidDigit);
                }
            },
        }
    }

    #[test]
    fn test_wrong_string_in_uv_index() {
        let index = "5/h99/2";

        let result = parse(index);

        match result {
            Ok(_) => assert!(false, "Parsing was expected to fail."),
            Err(err) => match err {
                IndexParseError::UnexpectedNumberOfElements(_) => {
                    assert!(false, "Parsing failed due to the wrong reason.")
                }
                IndexParseError::NumberParseError(parsed_line, reason) => {
                    assert_eq!(parsed_line, "h99");
                    assert_eq!(*reason.kind(), IntErrorKind::InvalidDigit);
                }
            },
        }
    }

    #[test]
    fn test_wrong_string_in_normal_index() {
        let index = "5/99/z2";

        let result = parse(index);

        match result {
            Ok(_) => assert!(false, "Parsing was expected to fail."),
            Err(err) => match err {
                IndexParseError::UnexpectedNumberOfElements(_) => {
                    assert!(false, "Parsing failed due to the wrong reason.")
                }
                IndexParseError::NumberParseError(parsed_line, reason) => {
                    assert_eq!(parsed_line, "z2");
                    assert_eq!(*reason.kind(), IntErrorKind::InvalidDigit);
                }
            },
        }
    }
}
