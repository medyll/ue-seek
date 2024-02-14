#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
use cpal::traits::HostTrait;
use cpal::traits::DeviceTrait;

// mod audio;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder
        ::default()
        .plugin(tauri_plugin_shell::init())
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
