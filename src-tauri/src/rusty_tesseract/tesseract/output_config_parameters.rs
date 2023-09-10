#![allow(warnings)]

use super::{
    command::{get_tesseract_command, run_tesseract_command},
    error::TessResult,
    parse_line_util::FromLine,
};
use core::fmt;

#[derive(Debug, PartialEq)]
pub struct ConfigParameterOutput {
    pub output: String,
    pub config_parameters: Vec<ConfigParameter>,
}

impl fmt::Display for ConfigParameterOutput {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.output)
    }
}

#[derive(Debug, PartialEq)]
pub struct ConfigParameter {
    pub name: String,
    pub default_value: String,
    pub description: String,
}

impl fmt::Display for ConfigParameter {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{} {} {}",
            self.name, self.default_value, self.description,
        )
    }
}

impl FromLine for ConfigParameter {
    fn from_line(line: &str) -> Option<Self> {
        let (name, x) = line.split_once("\t")?;
        let (default_value, description) = x.split_once("\t")?;

        Some(ConfigParameter {
            name: name.into(),
            default_value: default_value.into(),
            description: description.into(),
        })
    }
}

pub fn get_tesseract_config_parameters() -> TessResult<ConfigParameterOutput> {
    let mut command = get_tesseract_command()?;
    command.arg("--print-parameters");

    let output = run_tesseract_command(&mut command)?;

    let config_parameters = string_to_config_parameter_output(&output)?;

    Ok(ConfigParameterOutput {
        output,
        config_parameters,
    })
}

fn string_to_config_parameter_output(output: &str) -> TessResult<Vec<ConfigParameter>> {
    output
        .lines()
        .skip(1)
        .map(|line| ConfigParameter::parse(line))
        .collect::<_>()
}
