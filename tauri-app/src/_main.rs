// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
use cpal::traits::HostTrait;
use cpal::traits::DeviceTrait;

mod audio;

// use audio::register_speech;

/* fn main() {
    tauri_app_lib::run().plugin(tauri_plugin_window_state::Builder::default().build())
} */

fn main() {
    tauri::Builder
        ::default()
        .plugin(tauri_plugin_window_state::Builder::default().build())
        .plugin(tauri_plugin_fs::init())
        .invoke_handler(tauri::generate_handler![get_input_audio_list, get_output_audio_device])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

#[tauri::command]
fn get_input_audio_list() -> Vec<String> {
    println!("I was invoked from JS!");
    let host = cpal::default_host();
    let input_devices = host.input_devices().unwrap();

    let mut device_names = Vec::new();
    for device in input_devices {
        device_names.push(device.name().unwrap());
    }

    device_names
}

#[tauri::command]
fn get_output_audio_device() -> Vec<String> {
    let host = cpal::default_host();
    let output_devices = host.output_devices().unwrap();

    let mut device_names = Vec::new();
    for device in output_devices {
        device_names.push(device.name().unwrap());
    }

    device_names
}
