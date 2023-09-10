#![allow(warnings)]

use core::fmt;

use super::{
    command::{create_tesseract_command, run_tesseract_command},
    error::TessResult,
    input::{Args, Image},
    parse_line_util::{parse_next, FromLine},
};

#[derive(Debug, PartialEq)]
pub struct DataOutput {
    pub output: String,
    pub data: Vec<Data>,
}

impl fmt::Display for DataOutput {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.output)
    }
}

#[derive(Debug, PartialEq)]
pub struct Data {
    pub level: i32,
    pub page_num: i32,
    pub block_num: i32,
    pub par_num: i32,
    pub line_num: i32,
    pub word_num: i32,
    pub left: i32,
    pub top: i32,
    pub width: i32,
    pub height: i32,
    pub conf: f32,
    pub text: String,
}

impl fmt::Display for Data {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{} {} {} {} {} {} {} {} {} {} {} {}",
            self.level,
            self.page_num,
            self.block_num,
            self.par_num,
            self.line_num,
            self.word_num,
            self.left,
            self.top,
            self.width,
            self.height,
            self.conf,
            self.text,
        )
    }
}

impl FromLine for Data {
    fn from_line(line: &str) -> Option<Self> {
        let mut x = line.split_whitespace();
        Some(Data {
            level: parse_next(&mut x)?,
            page_num: parse_next(&mut x)?,
            block_num: parse_next(&mut x)?,
            par_num: parse_next(&mut x)?,
            line_num: parse_next(&mut x)?,
            word_num: parse_next(&mut x)?,
            left: parse_next(&mut x)?,
            top: parse_next(&mut x)?,
            width: parse_next(&mut x)?,
            height: parse_next(&mut x)?,
            conf: parse_next(&mut x)?,
            text: x.next().unwrap_or("").to_string(),
        })
    }
}

pub fn image_to_data(image: &Image, args: &Args) -> TessResult<DataOutput> {
    let mut command = create_tesseract_command(image, args)?;
    command.arg("tsv");

    let output = run_tesseract_command(&mut command)?;

    let data = string_to_data(&output)?;

    Ok(DataOutput { output, data })
}

fn string_to_data(output: &str) -> TessResult<Vec<Data>> {
    output
        .lines()
        .into_iter()
        .skip(1)
        .map(|line| Data::parse(line.into()))
        .collect::<_>()
}
