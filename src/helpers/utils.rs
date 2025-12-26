use std::fs;
use std::path::{Path, PathBuf};

use crate::models::enums::SerializableButton;

pub fn map_enigo_button(button: SerializableButton) -> enigo::Button {
    match button {
        SerializableButton::Left => enigo::Button::Left,
        SerializableButton::Right => enigo::Button::Right,
        SerializableButton::Middle => enigo::Button::Middle,
    }
}

pub fn map_rdev_button(button: rdev::Button) -> Option<SerializableButton> {
    match button {
        rdev::Button::Left => Some(SerializableButton::Left),
        rdev::Button::Right => Some(SerializableButton::Right),
        rdev::Button::Middle => Some(SerializableButton::Middle),
        _ => None,
    }
}

pub fn next_recording_path(name: &str) -> PathBuf {
    let base_dir = Path::new("records");
    let target_dir = base_dir.join(name);

    // Create the directory if it doesn't exist
    fs::create_dir_all(&target_dir).expect("Failed to create recording directory");

    let mut max_index = 0;

    // Read existing files in the directory
    if let Ok(entries) = fs::read_dir(&target_dir) {
        for entry in entries.flatten() {
            let file_name = entry.file_name();
            let file_name = file_name.to_string_lossy();

            // Search for files matching the pattern "<name>_<number>.json"
            let prefix = format!("{}_", name);

            if file_name.starts_with(&prefix) && file_name.ends_with(".json") {
                let number_part = &file_name[prefix.len()..file_name.len() - 5];

                if let Ok(num) = number_part.parse::<u32>() {
                    max_index = max_index.max(num);
                }
            }
        }
    }

    // Generate the next file name
    let new_index = max_index + 1;
    let file_name = format!("{}_{}.json", name, new_index);

    target_dir.join(file_name)
}

pub fn get_count_of_recordings(name: &str) -> u32 {
    let base_dir = Path::new("records");
    let target_dir = base_dir.join(name);

    // Create the directory if it doesn't exist
    fs::create_dir_all(&target_dir).expect("Failed to create recording directory");

    let mut max_index = 0;

    // Read existing files in the directory
    if let Ok(entries) = fs::read_dir(&target_dir) {
        for entry in entries.flatten() {
            let file_name = entry.file_name();
            let file_name = file_name.to_string_lossy();

            // Search for files matching the pattern "<name>_<number>.json"
            let prefix = format!("{}_", name);

            if file_name.starts_with(&prefix) && file_name.ends_with(".json") {
                let number_part = &file_name[prefix.len()..file_name.len() - 5];

                if let Ok(num) = number_part.parse::<u32>() {
                    max_index = max_index.max(num);
                }
            }
        }
    }

    // return the count of recordings
    max_index
}
