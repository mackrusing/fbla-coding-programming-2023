use serde_json::json;

use crate::types::events::Event;
use crate::types::students::Student;
use crate::types::grade_lvls::GradeLvl;

// GradeLvl tests

#[test]
fn grade9_to_json() {
    let to_json = serde_json::to_string(&GradeLvl::Nine).unwrap();
    let expected = json!(9).to_string();
    assert_eq!(to_json, expected);
}
#[test]
fn grade10_to_json() {
    let to_json = serde_json::to_string(&GradeLvl::Ten).unwrap();
    let expected = json!(10).to_string();
    assert_eq!(to_json, expected);
}
#[test]
fn grade11_to_json() {
    let to_json = serde_json::to_string(&GradeLvl::Eleven).unwrap();
    let expected = json!(11).to_string();
    assert_eq!(to_json, expected);
}
#[test]
fn grade12_to_json() {
    let to_json = serde_json::to_string(&GradeLvl::Twelve).unwrap();
    let expected = json!(12).to_string();
    assert_eq!(to_json, expected);
}

#[test]
fn grade9_from_json() {
    let from_json: GradeLvl = serde_json::from_str(&json!(9).to_string()).unwrap();
    let expected = GradeLvl::Nine;
    assert_eq!(from_json, expected);
}
#[test]
fn grade10_from_json() {
    let from_json: GradeLvl = serde_json::from_str(&json!(10).to_string()).unwrap();
    let expected = GradeLvl::Ten;
    assert_eq!(from_json, expected);
}
#[test]
fn grade11_from_json() {
    let from_json: GradeLvl = serde_json::from_str(&json!(11).to_string()).unwrap();
    let expected = GradeLvl::Eleven;
    assert_eq!(from_json, expected);
}
#[test]
fn grade12_from_json() {
    let from_json: GradeLvl = serde_json::from_str(&json!(12).to_string()).unwrap();
    let expected = GradeLvl::Twelve;
    assert_eq!(from_json, expected);
}

// Event tests

#[test]
fn event_to_json() {
    let to_json = serde_json::to_string(&Event {
        id: 1,
        points: 5,
    }).unwrap();
    let expected = json!({
        "id": 1,
        "points": 5
    }).to_string();
    assert_eq!(to_json, expected);
}

#[test]
fn event_from_json() {
    let from_json: Event = serde_json::from_str(&json!({
        "id": 1,
        "points": 5
    }).to_string()).unwrap();
    let expected = Event {
        id: 1,
        points: 5,
    };
    assert_eq!(from_json, expected);
}

// Student tests
// #[test]
// fn student_to_json() {
//     let to_json = serde_json::to_string(&Student {
//         id: 1,
//         first_name: String::from("Mack"),
//         last_name: String::from("Rusing"),
//         grade_lvl: GradeLvl::Eleven,
//         completed_events: vec![1, 2, 5]
//     }).unwrap();
//     let expected = json!({
//         "id": 1,
//         "first_name": "Mack",
//         "last_name": "Rusing",
//         "grade_lvl": 11,
//         "completed_events": [1, 2, 5]
//     }).to_string();
//     assert_eq!(to_json, expected);
// }
