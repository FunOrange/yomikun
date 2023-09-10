#![allow(warnings)]

use super::{
    command::{create_tesseract_command, run_tesseract_command},
    error::TessResult,
    input::{Args, Image},
    parse_line_util::{parse_next, FromLine},
};
use core::fmt;

#[derive(Debug, PartialEq)]
pub struct BoxOutput {
    pub output: String,
    pub boxes: Vec<Box>,
}

impl fmt::Display for BoxOutput {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.output)
    }
}

#[derive(Debug, PartialEq)]
pub struct Box {
    pub symbol: String,
    pub left: i32,
    pub bottom: i32,
    pub right: i32,
    pub top: i32,
    pub page: i32,
}

impl fmt::Display for Box {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{} {} {} {} {} {}",
            self.symbol, self.left, self.bottom, self.right, self.top, self.page
        )
    }
}

impl FromLine for Box {
    fn from_line(line: &str) -> Option<Self> {
        let mut x = line.split_whitespace();

        Some(Box {
            symbol: x.next()?.to_string(),
            left: parse_next(&mut x)?,
            bottom: parse_next(&mut x)?,
            right: parse_next(&mut x)?,
            top: parse_next(&mut x)?,
            page: parse_next(&mut x)?,
        })
    }
}

pub fn image_to_boxes(image: &Image, args: &Args) -> TessResult<BoxOutput> {
    let mut command = create_tesseract_command(image, args)?;
    command.arg("makebox");

    let output = run_tesseract_command(&mut command)?;
    let boxes = string_to_boxes(&output)?;
    Ok(BoxOutput { output, boxes })
}

fn string_to_boxes(output: &str) -> TessResult<Vec<Box>> {
    output
        .lines()
        .into_iter()
        .map(|line| Box::parse(line.into()))
        .collect::<_>()
}
