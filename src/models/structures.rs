use serde::{Deserialize, Serialize};

#[path = "../models/enums.rs"]
mod enums;

#[derive(Serialize, Deserialize, Clone, Copy, Debug)]
pub struct RecordedEvent {
    pub(crate) action_record: super::enums::ActionEnum,
    pub(crate) timestamp: i64,
}
