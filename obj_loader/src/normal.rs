use std::fmt::Debug;
use std::str::FromStr;

use math::Normal3;

#[derive(Debug)]
pub enum NormalParseError<T: FromStr> {
    WrongPrefix(String, String),
    UnexpectedNumberOfElements(String, usize),
    UnableToParseX(String, T::Err),
    UnableToParseY(String, T::Err),
    UnableToParseZ(String, T::Err),
}

pub fn parse<T: FromStr>(line: &str) -> Result<Normal3<T>, NormalParseError<T>>
where
    <T as FromStr>::Err: Debug,
{
    let split: Vec<&str> = line.split(" ").collect();

    if split.len() != 4 {
        return Err(NormalParseError::UnexpectedNumberOfElements(
            String::from(line),
            split.len(),
        ));
    }
    if split[0] != "vn" {
        return Err(NormalParseError::WrongPrefix(
            String::from(line),
            String::from(split[0]),
        ));
    }
    let x = split[1].parse();
    let y = split[2].parse();
    let z = split[3].parse();

    if x.is_err() {
        return Err(NormalParseError::UnableToParseX(
            String::from(line),
            x.err().unwrap(),
        ));
    }

    if y.is_err() {
        return Err(NormalParseError::UnableToParseY(
            String::from(line),
            y.err().unwrap(),
        ));
    }

    if y.is_err() {
        return Err(NormalParseError::UnableToParseZ(
            String::from(line),
            z.err().unwrap(),
        ));
    }

    Ok(Normal3::new(x.unwrap(), y.unwrap(), z.unwrap()))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_normal_f32() {
        let line = "vn 1.0 -2.0 3.0";
        let n = Normal3::new(1.0, -2.0, 3.0);

        let normal = parse::<f32>(line);

        assert!(normal.is_ok());
        assert_eq!(n, normal.unwrap());
    }

    #[test]
    fn parse_normal_f64() {
        let line = "vn 1.0 -2.0 3.0";
        let n = Normal3::new(1.0, -2.0, 3.0);

        let normal = parse::<f64>(line);

        assert!(normal.is_ok());
        assert_eq!(n, normal.unwrap());
    }
}
