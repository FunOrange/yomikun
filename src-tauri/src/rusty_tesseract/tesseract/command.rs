#![allow(warnings)]

use std::fs::{self, DirEntry};
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};
use std::string::ToString;

use super::error::{TessError, TessResult};
use super::input::{Args, Image};

#[cfg(target_os = "windows")]
use std::os::windows::process::CommandExt;

#[cfg(target_os = "windows")]
const CREATE_NO_WINDOW: u32 = 0x08000000;

static mut tesseract_path: Option<PathBuf> = None;

fn find_tesseract_exe(dir: &Path) -> Option<PathBuf> {
    if let Ok(entries) = fs::read_dir(dir) {
        for entry in entries {
            if let Ok(entry) = entry {
                let target_filename = if cfg!(target_os = "windows") {
                    "tesseract.exe"
                } else {
                    "tesseract"
                };
                let entry_path = entry.path();
                if entry_path.file_name() == Some(target_filename.as_ref()) {
                    return Some(entry_path);
                }
                if entry_path.is_dir() {
                    if let Some(result) = find_tesseract_exe(&entry_path) {
                        return Some(result);
                    }
                }
            }
        }
    }
    None
}

pub fn get_tesseract_command() -> TessResult<Command> {
    if unsafe { tesseract_path.is_none() } {
        // Look for tesseract.exe
        let tesseract_exe = find_tesseract_exe(Path::new("."));
        if (tesseract_exe.is_none()) {
            return Err(TessError::TesseractNotFoundError);
        } else {
            unsafe { tesseract_path = Some(tesseract_exe.unwrap()) }
        }
    }
    let tesseract = unsafe { tesseract_path.as_ref().unwrap().clone() };
    Ok(Command::new(tesseract))
}

pub fn get_tesseract_version() -> TessResult<String> {
    let mut command = get_tesseract_command()?;
    command.arg("--version");

    run_tesseract_command(&mut command)
}

pub fn get_tesseract_langs() -> TessResult<Vec<String>> {
    let mut command = get_tesseract_command()?;
    command.arg("--list-langs");

    let output = run_tesseract_command(&mut command)?;
    let langs = output.lines().skip(1).map(|x| x.into()).collect();
    Ok(langs)
}

pub fn run_tesseract_command(command: &mut Command) -> TessResult<String> {
    if cfg!(debug_assertions) {
        show_command(command);
    }

    #[cfg(target_os = "windows")]
    command.creation_flags(CREATE_NO_WINDOW);

    let child = command
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .map_err(|_| TessError::TesseractNotFoundError)?;

    let output = child
        .wait_with_output()
        .map_err(|_| TessError::TesseractNotFoundError)?;

    let out = String::from_utf8(output.stdout).unwrap();
    let err = String::from_utf8(output.stderr).unwrap();
    let status = output.status;

    match status.code() {
        Some(0) => Ok(out),
        _ => Err(TessError::CommandExitStatusError(status.to_string(), err)),
    }
}

fn show_command(command: &Command) {
    let params: Vec<String> = command
        .get_args()
        .map(|x| x.to_str().unwrap_or(""))
        .map(|s| s.to_string())
        .collect();

    println!(
        "Tesseract Command: {} {}",
        command.get_program().to_str().unwrap(),
        params.join(" ")
    );
}

pub fn image_to_string(image: &Image, args: &Args) -> TessResult<String> {
    let mut command = create_tesseract_command(image, args)?;
    let output = run_tesseract_command(&mut command)?;

    Ok(output)
}

pub fn create_tesseract_command(image: &Image, args: &Args) -> TessResult<Command> {
    let mut command = get_tesseract_command()?;
    command
        .arg(image.get_image_path()?)
        .arg("stdout")
        .arg("-l")
        .arg(args.lang.clone());

    // if let Some(dpi) = args.dpi {
    //     command.arg("--dpi").arg(dpi.to_string());
    // }

    if let Some(psm) = args.psm {
        command.arg("--psm").arg(psm.to_string());
    }

    if let Some(oem) = args.oem {
        command.arg("--oem").arg(oem.to_string());
    }

    if let Some(parameter) = args.get_config_variable_args() {
        command.arg("-c").arg(parameter);
    }

    Ok(command)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_tesseract_langs() {
        let langs = get_tesseract_langs().unwrap();

        assert!(langs.contains(&"eng".into()));
    }
}
