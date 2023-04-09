use serde::{Serialize, Deserialize};

#[derive(Debug, PartialEq)]
#[derive(Serialize, Deserialize)]
pub struct Event {
    pub id: u32,
    pub points: i32,
}
