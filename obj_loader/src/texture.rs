use std::fmt::Debug;
use std::str::FromStr;

use math::Point2;

#[derive(Debug)]
pub enum TextureParseError<T: FromStr> {
    WrongPrefix(String, String),
    UnexpectedNumberOfElements(String, usize),
    UnableToParseX(String, T::Err),
    UnableToParseY(String, T::Err),
}

pub fn parse<T: FromStr>(line: &str) -> Result<Point2<T>, TextureParseError<T>>
where
    <T as FromStr>::Err: Debug,
{
    let split: Vec<&str> = line.split(" ").collect();

    if split.len() != 3 {
        return Err(TextureParseError::UnexpectedNumberOfElements(
            String::from(line),
            split.len(),
        ));
    }
    if split[0] != "vt" {
        return Err(TextureParseError::WrongPrefix(
            String::from(line),
            String::from(split[0]),
        ));
    }
    let x = split[1].parse();
    let y = split[2].parse();

    if x.is_err() {
        return Err(TextureParseError::UnableToParseX(
            String::from(line),
            x.err().unwrap(),
        ));
    }

    if y.is_err() {
        return Err(TextureParseError::UnableToParseY(
            String::from(line),
            y.err().unwrap(),
        ));
    }

    Ok(Point2::new(x.unwrap(), y.unwrap()))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_texture_f32() {
        let line = "vt 1.0 -2.0";
        let t = Point2::new(1.0, -2.0);

        let tex = parse::<f32>(line);

        assert!(tex.is_ok());
        assert_eq!(t, tex.unwrap());
    }

    #[test]
    fn parse_texture_f64() {
        let line = "vt 1.0 -2.0";
        let t = Point2::new(1.0, -2.0);

        let tex = parse::<f64>(line);

        assert!(tex.is_ok());
        assert_eq!(t, tex.unwrap());
    }
}
