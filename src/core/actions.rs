use std::fs::File;
use std::io::Read;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::{Duration, Instant};

use chrono::Utc;
use enigo::{Coordinate, Direction, Enigo, Mouse, Settings};
use rdev::{Event, EventType, Key, listen};

// Personals
use crate::helpers::utils::{
    get_count_of_recordings, map_enigo_button, map_rdev_button, next_recording_path,
};
use crate::models::enums::ActionEnum;
use crate::models::structures::RecordedEvent;

pub fn record_mouse(file_name: &str) {
    println!("Recording mouse movements and clicks. Press Esc to stop.");

    // ::: Define Enigo settings and start recording :::
    let output_path = next_recording_path(file_name);
    let new_file_count = get_count_of_recordings(file_name) + 1;

    let events = Arc::new(Mutex::new(Vec::<RecordedEvent>::new()));
    let last_pos = Arc::new(Mutex::new((0.0_f64, 0.0_f64)));
    let start_time = Utc::now().timestamp_millis();

    let events_clone = events.clone();
    let last_pos_clone = last_pos.clone();

    thread::spawn(move || {
        if let Err(error) = listen(move |event: Event| {
            let timestamp = Utc::now().timestamp_millis() - start_time;
            let mut evs = events_clone.lock().unwrap();

            match event.event_type {
                // ::: Save the mouse position when moved :::
                EventType::MouseMove { x, y } => {
                    *last_pos_clone.lock().unwrap() = (x, y);
                    evs.push(RecordedEvent {
                        action_record: ActionEnum::Move { x, y },
                        timestamp,
                    });
                }

                // ::: Save the mouse position when clicked :::
                EventType::ButtonPress(button) => {
                    if let Some(btn) = map_rdev_button(button) {
                        let (x, y) = *last_pos_clone.lock().unwrap();

                        evs.push(RecordedEvent {
                            action_record: ActionEnum::ButtonPress { button: btn, x, y },
                            timestamp,
                        });
                    }
                }

                // ::: Save the mouse position when released :::
                EventType::ButtonRelease(button) => {
                    if let Some(btn) = map_rdev_button(button) {
                        let (x, y) = *last_pos_clone.lock().unwrap();

                        evs.push(RecordedEvent {
                            action_record: ActionEnum::ButtonRelease { button: btn, x, y },
                            timestamp,
                        });
                    }
                }

                // ::: Stop the recording and save the events :::
                EventType::KeyPress(Key::Escape) => {
                    let json = serde_json::to_string_pretty(&*evs).unwrap();

                    println!(
                        "\nRecording stopped. Saving the recording number #{}",
                        new_file_count.to_string()
                    );
                    std::fs::write(&output_path, json).unwrap();
                    std::process::exit(0);
                }

                _ => {}
            }
        }) {
            println!("Error listening to events: {:?}", error);
        }
    });

    // Main loop to keep the program running
    loop {
        thread::sleep(Duration::from_secs(1));
    }
}

pub fn replay_mouse(file_name: &str, file_number: &str) {
    println!("Reproducing recording...");

    // ::: Get the file number and the output path :::
    let file_number_used: u32 = if file_number.is_empty() {
        let max_count = get_count_of_recordings(file_name);
        rand::random::<u32>() % max_count + 1
    } else {
        file_number.parse::<u32>().unwrap()
    };

    // ::: Get the output path and the events :::
    let output_path = format!(
        "records/{}/{}_{}.json",
        file_name,
        file_name,
        file_number_used.to_string()
    );

    print!(
        "Using recording file: {} #{}\n",
        file_name, file_number_used
    );

    // ::: Import the recording from the JSON file :::
    let mut file = File::open(&output_path).expect("No file found. Please record first.");
    let mut json = String::new();
    file.read_to_string(&mut json).unwrap();

    let events: Vec<RecordedEvent> = serde_json::from_str(&json).expect("Invalid JSON format.");

    // ::: Check if the recording is empty :::
    if events.is_empty() {
        println!("The recording is empty. Please try to delete the file and record again.");
        return;
    }

    // ::: Define Enigo settings and start reproducing the events :::
    let settings = Settings::default();
    let mut enigo = Enigo::new(&settings).unwrap();

    let recording_start = events[0].timestamp;
    let start = Instant::now();

    // ::: Main loop of event reproduction :::
    for event in events {
        let target_time = Duration::from_millis((event.timestamp - recording_start) as u64);
        let elapsed = start.elapsed();

        // ::: Wait until the target time :::
        if target_time > elapsed {
            thread::sleep(target_time - elapsed);
        }

        // ::: Reproduce the event :::
        match event.action_record {
            // ::: Move the mouse :::
            ActionEnum::Move { x, y } => {
                let _ = enigo.move_mouse(x as i32, y as i32, Coordinate::Abs);
            }

            // ::: Press the mouse button :::
            ActionEnum::ButtonPress { button, .. } => {
                let _ = enigo.button(map_enigo_button(button), Direction::Press);
            }

            // ::: Release the mouse button :::
            ActionEnum::ButtonRelease { button, .. } => {
                let _ = enigo.button(map_enigo_button(button), Direction::Release);
            }
        }
    }

    println!("Reproduction finished!");
}
