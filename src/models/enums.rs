use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Copy, Debug)]
#[allow(dead_code)]
pub enum SerializableButton {
    Left,
    Right,
    Middle,
    ControlLeft,
}

#[derive(Serialize, Deserialize, Clone, Copy, Debug)]
#[serde(tag = "type")]
#[allow(dead_code)]
pub enum ActionEnum {
    Move {
        x: f64,
        y: f64,
    },
    ButtonPress {
        button: SerializableButton,
        x: f64,
        y: f64,
    },
    ButtonRelease {
        button: SerializableButton,
        x: f64,
        y: f64,
    },
    TimeAction {
        index_of_action: u32,
        is_final: bool,
    },
}
