use serde::{Serialize, Deserialize};

use super::grade_lvls::GradeLvl;

#[derive(Debug, Clone)]
#[derive(Serialize, Deserialize)]
pub struct Student {
    pub id: u32,
    pub first_name: String,
    pub last_name: String,
    pub grade_lvl: GradeLvl,
    pub completed_events: Vec<u32>, // event ids
}
