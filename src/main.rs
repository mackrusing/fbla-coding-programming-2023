#[macro_use] extern crate rocket;
use rocket::serde::json::Json;
// use rocket::response::status;
use rocket::State as AppState;
use std::sync::Mutex;
use activity_tracker::types::{Student, Event, GradeLvl};
use activity_tracker::state::State;

//
// /students
//

#[get("/students?<first_name>&<last_name>&<grade_lvl>")]
fn get_students(
    state: &AppState<Mutex<State>>,
    first_name: Option<&str>,
    last_name: Option<&str>,
    grade_lvl: Option<GradeLvl>,
) -> Json<Vec<Student>> {
    let state = state.lock().unwrap();
    let result = state.get_students(first_name, last_name, grade_lvl);
    Json(result)
}

#[post("/students?<first_name>&<last_name>&<grade_lvl>")]
fn post_students(
    state: &AppState<Mutex<State>>,
    first_name: &str,
    last_name: &str,
    grade_lvl: GradeLvl,
) -> Json<Student> {
    let mut state = state.lock().unwrap();
    let result = state.post_students(first_name, last_name, grade_lvl);
    Json(result)
}

//
// /students/<id>
//

#[get("/students/<id>")]
fn get_student(
    state: &AppState<Mutex<State>>,
    id: u32,
) -> Option<Json<Student>> {
    let state = state.lock().unwrap();
    let result = state.get_student(id);
    match result {
        Some(student) => Some(Json(student)),
        None => None
    }
}

#[put("/students/<id>?<first_name>&<last_name>&<grade_lvl>")]
fn put_student(
    state: &AppState<Mutex<State>>,
    id: u32,
    first_name: &str,
    last_name: &str,
    grade_lvl: GradeLvl,
) -> Json<Student> {
    let mut state = state.lock().unwrap();
    let result = state.put_student(id, first_name, last_name, grade_lvl);
    Json(result)
}

#[patch("/students/<id>?<first_name>&<last_name>&<grade_lvl>")]
fn patch_student(
    state: &AppState<Mutex<State>>,
    id: u32,
    first_name: Option<&str>,
    last_name: Option<&str>,
    grade_lvl: Option<GradeLvl>,
) -> Option<Json<Student>> {
    let mut state = state.lock().unwrap();
    let result = state.patch_student(id, first_name, last_name, grade_lvl);
    match result {
        Some(student) => Some(Json(student)),
        None => None
    }
}

#[delete("/students/<id>")]
fn delete_student(
    state: &AppState<Mutex<State>>,
    id: u32,
) -> Option<()> {
    let mut state = state.lock().unwrap();
    let result = state.delete_student(id);
    match result {
        Some(_) => Some(()),
        None => None
    }
}

//
// /students/<id>/completed_events
//

#[get("/students/<id>/completed_events")]
fn get_student_completed_events(
    state: &AppState<Mutex<State>>,
    id: u32,
) -> Option<Json<Vec<Event>>> {
    let state = state.lock().unwrap();
    let result = state.get_student_completed_events(id);
    match result {
        Some(events) => Some(Json(events)),
        None => None
    }
}

#[post("/students/<id>/completed_events?<event_id>")]
fn post_student_completed_events(
    state: &AppState<Mutex<State>>,
    id: u32,
    event_id: u32,
) -> Option<Json<Event>> {
    let mut state = state.lock().unwrap();
    let result = state.post_student_completed_events(id, event_id);
    match result {
        Some(event) => Some(Json(event)),
        None => None
    }
}

#[patch("/students/<id>/completed_events?<event_id>")]
fn patch_student_completed_events(
    state: &AppState<Mutex<State>>,
    id: u32,
    event_id: u32,
) -> Option<()> {
    let mut state = state.lock().unwrap();
    let result = state.patch_student_completed_events(id, event_id);
    match result {
        Some(_) => Some(()),
        None => None
    }
}

#[delete("/students/<id>/completed_events")]
fn delete_student_completed_events(
    state: &AppState<Mutex<State>>,
    id: u32,
) -> Option<()> {
    let mut state = state.lock().unwrap();
    let result = state.delete_student_completed_events(id);
    match result {
        Some(_) => Some(()),
        None => None
    }
}

//
// /events
//

//
// /events/id
//

// #[get("/test")]
// fn test(state: &AppState<Mutex<State>>) {
//     let mut state = state.lock().unwrap();
//     state.get_student_completed_events(1);
// }

#[launch]
fn rocket() -> _ {
    rocket::build()
        // .manage(Arc::new(Mutex::new(State::new("data/students.json", "data/events.json"))))
        .manage(Mutex::new(State::new("data/students.json", "data/events.json")))
        .mount("/api", routes![
            // /students
            get_students,
            post_students,

            // /students/<id>
            get_student,
            put_student,
            patch_student,
            delete_student,

            // /students/<id>/completed_events
            get_student_completed_events,
            post_student_completed_events,
            patch_student_completed_events,
            delete_student_completed_events,

            // test endpoints
            // test // remove before production
        ])
}
