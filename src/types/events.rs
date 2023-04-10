use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, PartialEq)]
#[derive(Serialize, Deserialize)]
pub struct Event {
    pub id: u32,
    pub name: String,
    pub points: i32,
}
