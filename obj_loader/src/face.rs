use crate::index;
use crate::index::IndexParseError;

use std::fmt::Debug;

#[derive(Debug, PartialEq)]
pub struct ParsedFace3 {
    pub a: i128,
    pub b: i128,
    pub c: i128,
    pub na: Option<i128>,
    pub nb: Option<i128>,
    pub nc: Option<i128>,
    pub uva: Option<i128>,
    pub uvb: Option<i128>,
    pub uvc: Option<i128>,
}

#[derive(Debug, PartialEq)]
pub enum FaceParseError {
    UnexpectedNumberOfElements(String, usize),
    WrongPrefix(String, String),
    UnableToParseA(String, IndexParseError),
    UnableToParseB(String, IndexParseError),
    UnableToParseC(String, IndexParseError),
}

pub fn parse(line: &str) -> Result<ParsedFace3, FaceParseError> {
    let split: Vec<&str> = line.split(' ').collect();

    if split.len() != 4 {
        return Err(FaceParseError::UnexpectedNumberOfElements(
            String::from(line),
            split.len(),
        ));
    }
    if split[0] != "f" {
        return Err(FaceParseError::WrongPrefix(
            String::from(line),
            String::from(split[0]),
        ));
    }

    let a = index::parse(split[1]);
    if a.is_err() {
        return Err(FaceParseError::UnableToParseA(
            String::from(split[1]),
            a.err().unwrap(),
        ));
    }
    let b = index::parse(split[2]);
    if b.is_err() {
        return Err(FaceParseError::UnableToParseB(
            String::from(split[2]),
            b.err().unwrap(),
        ));
    }
    let c = index::parse(split[3]);
    if c.is_err() {
        return Err(FaceParseError::UnableToParseC(
            String::from(split[3]),
            c.err().unwrap(),
        ));
    }

    let a = a.unwrap();
    let b = b.unwrap();
    let c = c.unwrap();

    Ok(ParsedFace3 {
        a: a.0,
        b: b.0,
        c: c.0,
        na: a.2,
        nb: b.2,
        nc: c.2,
        uva: a.1,
        uvb: b.1,
        uvc: c.1,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::num::IntErrorKind;

    #[test]
    fn test_parse_face_vertex_only() {
        let face = "f 5 99 7";

        let result = parse(face);

        assert_eq!(
            result,
            Ok(ParsedFace3 {
                a: 5,
                b: 99,
                c: 7,
                uva: None,
                uvb: None,
                uvc: None,
                na: None,
                nb: None,
                nc: None
            })
        );
    }

    #[test]
    fn test_parse_index_vertex_and_uv() {
        let face = "f 5/1 99/87 79/23";

        let result = parse(face);

        assert_eq!(
            result,
            Ok(ParsedFace3 {
                a: 5,
                b: 99,
                c: 79,
                uva: Some(1),
                uvb: Some(87),
                uvc: Some(23),
                na: None,
                nb: None,
                nc: None
            })
        );
    }

    #[test]
    fn test_parse_index_vertex_and_normal() {
        let face = "f 5//2 99//77 23//42";

        let result = parse(face);

        assert_eq!(
            result,
            Ok(ParsedFace3 {
                a: 5,
                b: 99,
                c: 23,
                uva: None,
                uvb: None,
                uvc: None,
                na: Some(2),
                nb: Some(77),
                nc: Some(42)
            })
        );
    }

    #[test]
    fn test_parse_index_vertex_uv_and_normal() {
        let face = "f 5/99/2 4224/2345/11 633/13/63";

        let result = parse(face);

        assert_eq!(
            result,
            Ok(ParsedFace3 {
                a: 5,
                b: 4224,
                c: 633,
                uva: Some(99),
                uvb: Some(2345),
                uvc: Some(13),
                na: Some(2),
                nb: Some(11),
                nc: Some(63)
            })
        );
    }

    #[test]
    fn test_parse_negative() {
        let face = "f -5/-99/-2 -42/-23/-764 -1/-2/-3";

        let result = parse(face);

        assert_eq!(
            result,
            Ok(ParsedFace3 {
                a: -5,
                b: -42,
                c: -1,
                uva: Some(-99),
                uvb: Some(-23),
                uvc: Some(-2),
                na: Some(-2),
                nb: Some(-764),
                nc: Some(-3)
            })
        );
    }

    #[test]
    fn test_fails_on_emtpty() {
        let face = "";

        let result = parse(face);

        assert_eq!(
            result,
            Err(FaceParseError::UnexpectedNumberOfElements(
                String::from(face),
                1
            ))
        );
    }

    #[test]
    fn test_fails_on_to_many_elements() {
        let face = "f 5 2 5 9";

        let result = parse(face);

        assert_eq!(
            result,
            Err(FaceParseError::UnexpectedNumberOfElements(
                String::from(face),
                5
            ))
        );
    }

    #[test]
    fn test_fails_on_wrong_prefix() {
        let face = "v 7 5 6";

        let result = parse(face);

        assert_eq!(
            result,
            Err(FaceParseError::WrongPrefix(
                String::from(face),
                String::from("v")
            ))
        );
    }

    #[test]
    fn test_wrong_string_in_vertex_index() {
        let face = "f l5/99/2 5/99/3 5/99/4";

        let result = parse(face);

        assert!(matches!(result, Err(FaceParseError::UnableToParseA(_, _))));
        if let Err(FaceParseError::UnableToParseA(_, idx_err)) = result {
            match idx_err {
                IndexParseError::NumberParseError(parsed_line, reason) => {
                    assert_eq!(parsed_line, "l5");
                    assert_eq!(*reason.kind(), IntErrorKind::InvalidDigit);
                }
                _ => panic!("Expected NumberParseError"),
            }
        } else {
            panic!("Expected UnableToParseA error");
        }
    }

    #[test]
    fn test_wrong_string_in_uv_index() {
        let face = "f 5/h99/2 5/99/3 5/99/4";

        let result = parse(face);

        assert!(matches!(result, Err(FaceParseError::UnableToParseA(_, _))));
        if let Err(FaceParseError::UnableToParseA(_, idx_err)) = result {
            match idx_err {
                IndexParseError::NumberParseError(parsed_line, reason) => {
                    assert_eq!(parsed_line, "h99");
                    assert_eq!(*reason.kind(), IntErrorKind::InvalidDigit);
                }
                _ => panic!("Expected NumberParseError"),
            }
        } else {
            panic!("Expected UnableToParseA error");
        }
    }

    #[test]
    fn test_wrong_string_in_normal_index() {
        let face = "f 5/99/z2 5/99/3 5/99/4";

        let result = parse(face);

        assert!(matches!(result, Err(FaceParseError::UnableToParseA(_, _))));
        if let Err(FaceParseError::UnableToParseA(_, idx_err)) = result {
            match idx_err {
                IndexParseError::NumberParseError(parsed_line, reason) => {
                    assert_eq!(parsed_line, "z2");
                    assert_eq!(*reason.kind(), IntErrorKind::InvalidDigit);
                }
                _ => panic!("Expected NumberParseError"),
            }
        } else {
            panic!("Expected UnableToParseA error");
        }
    }

    #[test]
    fn test_wrong_string_in_vertex_index_b() {
        let face = "f 5/99/2 l5/99/3 5/99/4";

        let result = parse(face);

        assert!(matches!(result, Err(FaceParseError::UnableToParseB(_, _))));
        if let Err(FaceParseError::UnableToParseB(_, idx_err)) = result {
            match idx_err {
                IndexParseError::NumberParseError(parsed_line, reason) => {
                    assert_eq!(parsed_line, "l5");
                    assert_eq!(*reason.kind(), IntErrorKind::InvalidDigit);
                }
                _ => panic!("Expected NumberParseError"),
            }
        } else {
            panic!("Expected UnableToParseB error");
        }
    }

    #[test]
    fn test_wrong_string_in_uv_index_b() {
        let face = "f 5/99/2 5/h99/3 5/99/4";

        let result = parse(face);

        assert!(matches!(result, Err(FaceParseError::UnableToParseB(_, _))));
        if let Err(FaceParseError::UnableToParseB(_, idx_err)) = result {
            match idx_err {
                IndexParseError::NumberParseError(parsed_line, reason) => {
                    assert_eq!(parsed_line, "h99");
                    assert_eq!(*reason.kind(), IntErrorKind::InvalidDigit);
                }
                _ => panic!("Expected NumberParseError"),
            }
        } else {
            panic!("Expected UnableToParseB error");
        }
    }

    #[test]
    fn test_wrong_string_in_normal_index_b() {
        let face = "f 5/99/2 5/99/z3 5/99/4";

        let result = parse(face);

        assert!(matches!(result, Err(FaceParseError::UnableToParseB(_, _))));
        if let Err(FaceParseError::UnableToParseB(_, idx_err)) = result {
            match idx_err {
                IndexParseError::NumberParseError(parsed_line, reason) => {
                    assert_eq!(parsed_line, "z3");
                    assert_eq!(*reason.kind(), IntErrorKind::InvalidDigit);
                }
                _ => panic!("Expected NumberParseError"),
            }
        } else {
            panic!("Expected UnableToParseB error");
        }
    }

    #[test]
    fn test_wrong_string_in_vertex_index_c() {
        let face = "f 5/99/2 5/99/3 l5/99/4";

        let result = parse(face);

        assert!(matches!(result, Err(FaceParseError::UnableToParseC(_, _))));
        if let Err(FaceParseError::UnableToParseC(_, idx_err)) = result {
            match idx_err {
                IndexParseError::NumberParseError(parsed_line, reason) => {
                    assert_eq!(parsed_line, "l5");
                    assert_eq!(*reason.kind(), IntErrorKind::InvalidDigit);
                }
                _ => panic!("Expected NumberParseError"),
            }
        } else {
            panic!("Expected UnableToParseC error");
        }
    }

    #[test]
    fn test_wrong_string_in_uv_index_c() {
        let face = "f 5/99/2 5/99/3 5/h99/4";

        let result = parse(face);

        assert!(matches!(result, Err(FaceParseError::UnableToParseC(_, _))));
        if let Err(FaceParseError::UnableToParseC(_, idx_err)) = result {
            match idx_err {
                IndexParseError::NumberParseError(parsed_line, reason) => {
                    assert_eq!(parsed_line, "h99");
                    assert_eq!(*reason.kind(), IntErrorKind::InvalidDigit);
                }
                _ => panic!("Expected NumberParseError"),
            }
        } else {
            panic!("Expected UnableToParseC error");
        }
    }

    #[test]
    fn test_wrong_string_in_normal_index_c() {
        let face = "f 5/99/2 5/99/3 5/99/z4";

        let result = parse(face);

        assert!(matches!(result, Err(FaceParseError::UnableToParseC(_, _))));
        if let Err(FaceParseError::UnableToParseC(_, idx_err)) = result {
            match idx_err {
                IndexParseError::NumberParseError(parsed_line, reason) => {
                    assert_eq!(parsed_line, "z4");
                    assert_eq!(*reason.kind(), IntErrorKind::InvalidDigit);
                }
                _ => panic!("Expected NumberParseError"),
            }
        } else {
            panic!("Expected UnableToParseC error");
        }
    }
}
