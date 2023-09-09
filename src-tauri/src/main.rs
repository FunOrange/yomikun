// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::collections::HashMap;

use rusty_tesseract::{Args, Image};

fn main() {
    // 1. Read image
    let img = Image::from_path("img/string.png");

    // 2. Set tesseract parameters
    let default_args = Args::default();

    // the default parameters are
    /*
    Args {
        lang: "eng",
        dpi: Some(150),
        psm: Some(3),
        oem: Some(3),
    }
    */

    // fill your own argument struct if needed
    // Optional arguments are ignored if set to `None`
    let mut my_args = Args {
        //model language (tesseract default = 'eng')
        //available languages can be found by running 'rusty_tesseract::get_tesseract_langs()'
        lang: "eng".to_string(),

        //map of config variables
        //this example shows a whitelist for the normal alphabet. Multiple arguments are allowed.
        //available arguments can be found by running 'rusty_tesseract::get_tesseract_config_parameters()'
        config_variables: HashMap::from([(
            "tessedit_char_whitelist".into(),
            "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ".into(),
        )]),
        dpi: Some(150), // specify DPI for input image
        psm: Some(6), // define page segmentation mode 6 (i.e. "Assume a single uniform block of text")
        oem: Some(3), // define optical character recognition mode 3 (i.e. "Default, based on what is available")
    };

    tauri::Builder::default()
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
