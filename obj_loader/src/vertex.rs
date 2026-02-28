use std::fmt::Debug;
use std::str::FromStr;

use math::Point3;

#[derive(Debug)]
pub enum VertexParseError<T: FromStr> {
    WrongPrefix(String, String),
    UnexpectedNumberOfElements(String, usize),
    UnableToParseX(String, T::Err),
    UnableToParseY(String, T::Err),
    UnableToParseZ(String, T::Err),
}

pub fn parse<T: FromStr>(line: &str) -> Result<Point3<T>, VertexParseError<T>>
where
    <T as FromStr>::Err: Debug,
{
    let split: Vec<&str> = line.split(" ").collect();

    if split.len() != 4 {
        return Err(VertexParseError::UnexpectedNumberOfElements(
            String::from(line),
            split.len(),
        ));
    }
    if split[0] != "v" {
        return Err(VertexParseError::WrongPrefix(
            String::from(line),
            String::from(split[0]),
        ));
    }
    let x = split[1].parse();
    let y = split[2].parse();
    let z = split[3].parse();

    if x.is_err() {
        return Err(VertexParseError::UnableToParseX(
            String::from(line),
            x.err().unwrap(),
        ));
    }

    if y.is_err() {
        return Err(VertexParseError::UnableToParseY(
            String::from(line),
            y.err().unwrap(),
        ));
    }

    if z.is_err() {
        return Err(VertexParseError::UnableToParseZ(
            String::from(line),
            z.err().unwrap(),
        ));
    }

    Ok(Point3::new(x.unwrap(), y.unwrap(), z.unwrap()))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_vertex_f32() {
        let line = "v 1.0 -2.0 3.0";
        let p = Point3::new(1.0, -2.0, 3.0);

        let vertex = parse::<f32>(line);

        assert!(vertex.is_ok());
        assert_eq!(p, vertex.unwrap());
    }

    #[test]
    pub fn parse_vertex_f64() {
        let line = "v 1.0 -2.0 3.0";
        let n = Point3::new(1.0, -2.0, 3.0);

        let vertex = parse::<f64>(line);

        assert!(vertex.is_ok());
        assert_eq!(n, vertex.unwrap());
    }
}
