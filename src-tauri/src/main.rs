#![allow(warnings)]
// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::collections::HashMap;
use std::env;
use std::path::PathBuf;
mod rusty_tesseract;
use rusty_tesseract::command::image_to_string;
use rusty_tesseract::input::{Args, Image};
use rusty_tesseract::output_boxes;
use rusty_tesseract::tesseract::error::TessError;
use tauri::App;

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
    let docs_window = tauri::WindowBuilder::new(
        &handle,
        "external", /* the unique window label */
        tauri::WindowUrl::External("https://tauri.app/".parse().unwrap()),
    )
    .build()
    .unwrap();
}

#[tauri::command]
fn ipc_ocr(psm: i32) -> String {
    // 1. Read image
    let img_path = "shounen-no-abyss-3.png";
    let img_result = Image::from_path(img_path);
    let img = img_result.expect(format!("Failed to read image at path: {}", img_path).as_str());

    // 2. Set tesseract parameters
    // if (OCRlanguage === "japaneseVertical") {
    //     config = {
    //         lang: "jpn_vert",
    //         oem: 1,
    //         psm: 5,
    //     }
    // }
    // else if (OCRlanguage === "japaneseHorizontal") {
    //     config = {
    //         lang: "jpn",
    //         oem: 1,
    //         psm: 6,
    //     }
    // }
    let my_args = Args {
        lang: "jpn_vert".to_string(),
        config_variables: HashMap::new(),
        dpi: None,
        psm: Some(psm),
        oem: Some(1),
    };

    // 3. Get the tesseract model output
    // let output = image_to_string(&img, &my_args);
    // let output = match output {
    //     Ok(output) => output,
    //     Err(err) => match err {
    //         TessError::TesseractNotFoundError => {
    //             println!(
    //                 "Could not find tesseract executable at {:?}",
    //                 PathBuf::from(env::current_dir().unwrap())
    //                     .join("Tesseract-OCR")
    //                     .join("tesseract.exe")
    //             );
    //             String::new()
    //         }
    //         _ => panic!("WTF! {:?}", err),
    //     },
    // };

    // image_to_boxes creates a BoxOutput containing the parsed output from Tesseract when using the "makebox" Parameter
    let box_output = output_boxes::image_to_boxes(&img, &my_args).unwrap();
    println!(
        "The first boxfile symbol is: {}",
        box_output.boxes[0].symbol
    );
    println!("The full boxfile output is:\n{}", box_output.output);

    // image_to_data creates a DataOutput containing the parsed output from Tesseract when using the "TSV" Parameter
    // start_time = Instant::now();
    // let data_output = output_boxes::image_to_data(&img, &my_args).unwrap();
    // print_time_elapsed(start_time);
    // let first_text_line = &data_output.data[4];
    // println!(
    //     "The first text is '{}' with confidence {}",
    //     first_text_line.text, first_text_line.conf
    // );
    // println!("The full data output is:\n{}", data_output.output);
    box_output.output
}
