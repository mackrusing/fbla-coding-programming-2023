#[macro_use] extern crate rocket;
use rocket::serde::json::Json;
use rocket::response::status;
use rocket::State as AppState;
// use rocket::response::status;
use std::sync::Mutex;

use activity_tracker::types::{Student, Event, GradeLvl};
use activity_tracker::state::State;

#[get("/students?<first_name>&<last_name>&<grade_lvl>")]
fn get_students(
    state: &AppState<Mutex<State>>,
    first_name: Option<&str>,
    last_name: Option<&str>,
    grade_lvl: Option<GradeLvl>,
) -> Json<Vec<Student>> {
    let mut state = state.lock().unwrap();
    let result = state.get_students(first_name, last_name, grade_lvl);
    Json(result)
}

#[post("/students?<first_name>&<last_name>&<grade_lvl>")]
fn add_student(
    state: &AppState<Mutex<State>>,
    first_name: &str,
    last_name: &str,
    grade_lvl: GradeLvl,
) -> Json<Student> {
    let mut state = state.lock().unwrap();
    let result = state.post_student(first_name, last_name, grade_lvl);
    Json(result)
}

#[get("/students/<id>")]
fn get_student(
    state: &AppState<Mutex<State>>,
    id: u32,
) -> Option<Json<Student>> {
    let mut state = state.lock().unwrap();
    let result = state.get_student(id);
    match result {
        Some(student) => Some(Json(student)),
        None => None
    }
}

// #[put("/students/<id>")]
// fn get_student(
//     state: &AppState<Mutex<State>>,
//     id: u32,
// ) -> Option<Json<Student>> {
//     let mut state = state.lock().unwrap();
//     let result = state.get_student(id);
//     match result {
//         Ok(student) => Some(Json(student)),
//         Err(()) => None
//     }
// }

#[launch]
fn rocket() -> _ {
    rocket::build()
        // .manage(Arc::new(Mutex::new(State::new("data/students.json", "data/events.json"))))
        .manage(Mutex::new(State::new("data/students.json", "data/events.json")))
        .mount("/api", routes![
            get_students,
            add_student,
            get_student,
        ])
}
