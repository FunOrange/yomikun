#![allow(warnings)]
// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::collections::HashMap;
use std::env;
use std::path::PathBuf;
mod rusty_tesseract;
use rusty_tesseract::tesseract::error::TessError;

use active_win_pos_rs::get_active_window;
use image::RgbaImage;
use regex::Regex;
use win_screenshot::prelude::*;

use std::time::Instant;

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![ipc_create_window, ipc_ocr])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

fn print_time_elapsed(start_time: Instant) {
    println!(
        "Function took {} milliseconds to run",
        (Instant::now() - start_time).as_millis()
    );
}

#[tauri::command]
async fn ipc_create_window(handle: tauri::AppHandle) {
    let active_window = get_active_window().unwrap();
    let re = Regex::new(r"HWND\((\d+)\)").unwrap();
    let captures = re.captures(active_window.window_id.as_str()).unwrap();
    let integer_str = captures.get(1).unwrap();
    let hwnd = integer_str.as_str().parse::<isize>().unwrap();

    let using = Using::PrintWindow;
    let area = Area::ClientOnly;
    let buf = capture_window_ex(hwnd, using, area, None, None).unwrap();
    let img = RgbaImage::from_raw(buf.width, buf.height, buf.pixels).unwrap();
    img.save("screenshot.jpg").unwrap();

    #[cfg(dev)]
    let docs_window = tauri::WindowBuilder::new(
        &handle,
        "local",
        tauri::WindowUrl::External("http://localhost:3000/capture".parse().unwrap()),
    )
    .build()
    .unwrap();
    #[cfg(not(dev))]
    let docs_window = tauri::WindowBuilder::new(
        &handle,
        "local",
        tauri::WindowUrl::App("capture.html".into()),
    )
    .build()
    .unwrap();
}

#[tauri::command]
fn ipc_ocr(lang: String, psm: i32) -> String {
    // 1. Read image
    let img_path = "shounen-no-abyss-3.png";
    let img_result = rusty_tesseract::input::Image::from_path(img_path);
    let img = img_result.expect(format!("Failed to read image at path: {}", img_path).as_str());

    // 2. Set tesseract parameters
    let my_args = rusty_tesseract::input::Args {
        lang: lang,
        config_variables: HashMap::new(),
        dpi: None,
        psm: Some(psm),
        oem: Some(1),
    };

    // 3. Read data
    let start_time = Instant::now();
    let data_output = rusty_tesseract::output_data::image_to_data(&img, &my_args).unwrap();
    print_time_elapsed(start_time);
    data_output.output
}
