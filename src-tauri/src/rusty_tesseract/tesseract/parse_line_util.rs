use super::error::{TessError, TessResult};

pub fn parse_next<T: std::str::FromStr>(iter: &mut std::str::SplitWhitespace<'_>) -> Option<T> {
    iter.next()?.parse::<T>().ok()
}

pub trait FromLine: Sized {
    fn from_line(line: &str) -> Option<Self>;

    fn parse(line: &str) -> TessResult<Self> {
        Self::from_line(line).ok_or(TessError::ParseError(format!("invalid line '{}'", line)))
    }
}
