// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use rand::{thread_rng, Rng};

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet() -> String {
    format!("{}", make_scroll_name())
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![greet])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

fn make_scroll_name() -> String {
    let mut rng = thread_rng();
    let length = 4 + rng.gen_range(0..=6);
    let mut name = "zwój ".to_string();

    for i in 0..length {
        if i % 2 == 0 {
            name += match rng.gen_range(1..=27) {
                1 => "b",
                2 => "c",
                3 => "ch",
                4 => "cz",
                5 => "d",
                6 => "dz",
                7 => "f",
                8 => "g",
                9 => "h",
                10 => "j",
                11 => "k",
                12 => "l",
                13 => "m",
                14 => "n",
                15 => "p",
                16 => "r",
                17 => "rz",
                18 => "s",
                19 => "sz",
                20 => "t",
                21 => "w",
                22 => "z",
                23 => "dzi",
                24 => "si",
                25 => "zi",
                26 => "ni",
                _ => "ci"
            }
        } else {
            name += match rng.gen_range(1..=8) {
                1 => "a",
                2 => "e",
                3 => "en",
                4 => "i",
                5 => "o",
                6 => "on",
                7 => "u",
                _ => "y"
            }
        }
    }

    name
}
