use crate::types::{Student, Event, GradeLvl};

use std::fs;

#[derive(Debug)]
pub struct State {
    students: Vec<Student>,
    events: Vec<Event>,
    last_student_id: u32,
    last_event_id: u32,
    students_file: String,
    events_file: String,
}

impl State {
    // helpers
    fn next_student_id(&mut self) -> u32 {
        self.last_student_id += 1;
        self.last_student_id
    }
    fn next_event_id(&mut self) -> u32 {
        self.last_event_id += 1;
        self.last_event_id
    }
    // fn get_mut_student_from_students(&mut self, id: u32) -> Option<&mut Student> {
    //     self.students.iter_mut().find(|student| student.id == id)
    // }
    // fn get_student_from_students(&self, id: u32) -> Option<&Student> {
    //     self.students.iter().find(|student| student.id == id)
    // }

    //
    // student functions
    //   all return values are cloned
    //   all lists returned as vecs
    //   operations that can fail return an option
    //

    // GET /students
    pub fn get_students(&self, first_name: Option<&str>, last_name: Option<&str>, grade_lvl: Option<GradeLvl>) -> Vec<Student> {
        self.students.iter().filter_map(|student| {
            if let Some(first) = first_name { if student.first_name != first { return None; } }
            if let Some(last) = last_name { if student.last_name != last { return None; } }
            if let Some(grade) = &grade_lvl { if &student.grade_lvl != grade { return None; } }
            Some(student.clone())
        }).collect()
    }

    // POST /students
    pub fn post_student(&mut self, first_name: &str, last_name: &str, grade_lvl: GradeLvl) -> Student {
        let new_student = Student {
            id: self.next_student_id(),
            first_name: String::from(first_name),
            last_name: String::from(last_name),
            grade_lvl: grade_lvl,
            completed_events: Vec::new(),
        };

        self.students.push(new_student.clone());
        self.write_json();

        new_student
    }

    // GET /students/<id>
    pub fn get_student(&self, id: u32) -> Option<Student> {
        self.students.iter().find_map(|student| {
            if student.id == id { Some(student.clone()) }
            else { None }
        })
    }

    // PUT /students/<id>
    pub fn put_student(&mut self, id: u32, first_name: &str, last_name: &str, grade_lvl: GradeLvl) -> Student {
        let new_student = Student {
            id: id,
            first_name: String::from(first_name),
            last_name: String::from(last_name),
            grade_lvl: grade_lvl,
            completed_events: Vec::new(),
        };

        match self.students.iter().position(|student| student.id == id) {
            Some(i) => {
                self.students.remove(i);
                self.students.insert(i, new_student.clone());
            }
            None => {
                self.students.push(new_student.clone());
            }
        }

        self.write_json();
        new_student
    }

    // PATCH /students/<id>
    pub fn patch_student(&mut self, id: u32, first_name: Option<&str>, last_name: Option<&str>, grade_lvl: Option<GradeLvl>) -> Option<Student> {
        
        let mut student = self.students.iter_mut().find(|student| student.id == id)?;

        if let Some(first) = first_name { student.first_name = String::from(first); }
        if let Some(last) = last_name { student.last_name = String::from(last); }
        if let Some(grade) = grade_lvl { student.grade_lvl = grade; }

        let mut result = student.clone();
        self.write_json();
        Some(result)
    }

    // DELETE /students/<id>
    pub fn delete_student(&mut self, id: u32) -> Option<()> {
        // let mut found = false;
        let i = self.students.iter().position(|student| student.id == id)?;

        self.students.remove(i);
        self.write_json();
        Some(())
    }

    // GET /students/<id>/completed_events
    pub fn get_student_completed_events(&self, id: u32) -> Option<Vec<Event>> {
        let event_ids = &self.students.iter().find(|student| student.id == id)?.completed_events;
        // let result: Vec<Event> = self.events.iter().filter_map(|event| {
        //     for id in event_ids {
        //         if event.id == *id {
        //             return Some(event.clone());
        //         }
        //     }
        //     None
        // }).collect();
        let matches: Vec<&Event> = &self.events.iter().filter(|event| {
            for id in event_ids {
                if event.id == id {
                    return true;
                }
            }
            false
        }).collect();

        let result: Vec<Event> = matches.iter().map(|event| { event.clone(); }).collect();

        Some(result)

        // self.students.iter().filter_map(|student| {
        //     if let Some(first) = first_name { if student.first_name != first { return None; } }
        //     if let Some(last) = last_name { if student.last_name != last { return None; } }
        //     if let Some(grade) = &grade_lvl { if &student.grade_lvl != grade { return None; } }
        //     Some(student.clone())
        // }).collect()
    }

    //
    // read an write to fs
    //

    fn read_json(&mut self) {
        let students_string = fs::read_to_string(&self.students_file).expect("can't read students file");
        let events_string = fs::read_to_string(&self.events_file).expect("can't read events file");

        let parsed_students: Vec<Student> = serde_json::from_str(&students_string).expect("can't parse students from json");
        let parsed_events: Vec<Event> = serde_json::from_str(&events_string).expect("can't parse events from json");

        self.students = parsed_students;
        self.events = parsed_events;
    }
    fn write_json(&self) {
        let string_students = serde_json::to_string(&self.students).expect("can't parse students to json");
        let string_events = serde_json::to_string(&self.events).expect("can't parse events to json");

        fs::write(&self.students_file, string_students).expect("can't write to students file");
        fs::write(&self.events_file, string_events).expect("can't write to events file");
    }

    //
    // constructors
    //

    pub fn new(students_file: &str, events_file: &str) -> State {
        let mut state = State {
            students: Vec::new(),
            events: Vec::new(),
            last_student_id: 0,
            last_event_id: 0,
            students_file: String::from(students_file),
            events_file: String::from(events_file)
        };

        state.read_json();

        let mut highest_student_id = 0;
        for student in &state.students {
            if student.id > highest_student_id {
                highest_student_id = student.id;
            }
        }

        let mut highest_event_id = 0;
        for event in &state.events {
            if event.id > highest_event_id {
                highest_event_id = event.id;
            }
        }

        state.last_student_id = highest_student_id;
        state.last_event_id = highest_event_id;

        state
    }
}
