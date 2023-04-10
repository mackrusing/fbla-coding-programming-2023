use crate::types::{Student, Event, GradeLvl};
use serde_json::json;
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
    //
    // helpers
    //

    fn next_student_id(&mut self) -> u32 {
        self.last_student_id += 1;
        self.last_student_id
    }

    // fn next_event_id(&mut self) -> u32 {
    //     self.last_event_id += 1;
    //     self.last_event_id
    // }

    //
    // student functions
    //   all return values are cloned
    //   all lists returned as vecs
    //   operations that can fail return an option
    //

    //
    // students
    //

    pub fn get_students(&self, first_name: Option<&str>, last_name: Option<&str>, grade_lvl: Option<GradeLvl>) -> Vec<Student> {
        self.students.iter().filter_map(|student| {
            if let Some(first) = first_name { if student.first_name != first { return None; } }
            if let Some(last) = last_name { if student.last_name != last { return None; } }
            if let Some(grade) = &grade_lvl { if &student.grade_lvl != grade { return None; } }
            Some(student.clone())
        }).collect()
    }

    pub fn post_students(&mut self, first_name: &str, last_name: &str, grade_lvl: GradeLvl) -> Student {
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


    //
    // student
    //

    pub fn get_student(&self, id: u32) -> Option<Student> {
        self.students.iter().find_map(|student| {
            if student.id == id { Some(student.clone()) }
            else { None }
        })
    }

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

    pub fn patch_student(&mut self, id: u32, first_name: Option<&str>, last_name: Option<&str>, grade_lvl: Option<GradeLvl>) -> Option<Student> {
        
        let mut student = self.students.iter_mut().find(|student| student.id == id)?;

        if let Some(first) = first_name { student.first_name = String::from(first); }
        if let Some(last) = last_name { student.last_name = String::from(last); }
        if let Some(grade) = grade_lvl { student.grade_lvl = grade; }

        let result = student.clone();
        self.write_json();
        Some(result)
    }

    pub fn delete_student(&mut self, id: u32) -> Option<()> {
        let i = self.students.iter().position(|student| student.id == id)?;

        self.students.remove(i);
        self.write_json();
        Some(())
    }

    //
    // student completed events
    //

    pub fn get_student_completed_events(&self, id: u32) -> Option<Vec<Event>> {
        let event_ids = &self.students.iter().find(|student| student.id == id)?.completed_events;
        Some(self.events.iter().filter_map(|event| {
            if event_ids.contains(&event.id) {
                Some(event.clone())
            } else {
                None
            }
        }).collect())
    }

    pub fn post_student_completed_events(&mut self, id: u32, event_id: u32) -> Option<Event> {
        let selected_event = self.events.iter().find(|event| event.id == event_id)?;
        let selected_student = self.students.iter_mut().find(|student| student.id == id)?;
        if selected_student.completed_events.contains(&event_id) { return None }
        selected_student.completed_events.push(event_id);
        self.write_json();
        Some(selected_event.clone())
    }

    pub fn patch_student_completed_events(&mut self, id: u32, event_id: u32) -> Option<()> {
        let selected_student = self.students.iter_mut().find(|student| student.id == id)?;
        let i = selected_student.completed_events.iter().position(|existing_id| existing_id == &event_id)?;
        selected_student.completed_events.remove(i);
        self.write_json();
        Some(())
    }

    pub fn delete_student_completed_events(&mut self, id: u32) -> Option<()> {
        let mut selected_student = self.students.iter_mut().find(|student| student.id == id)?;
        selected_student.completed_events = Vec::new();
        self.write_json();
        Some(())
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

impl State {
    //
    // testing utility functions
    //

    pub fn util_reset_test_data(&mut self) {
        // students data
        let students_json = json!([
            {
                "id": 1,
                "first_name": "Charlie",
                "last_name": "Spring",
                "grade_lvl": 10,
                "completed_events": []
            },
            {
                "id": 2,
                "first_name": "Nick",
                "last_name": "Nelson",
                "grade_lvl": 10,
                "completed_events": []
            },
            {
                "id": 3,
                "first_name": "Elle",
                "last_name": "Argent",
                "grade_lvl": 11,
                "completed_events": []
            },
            {
                "id": 4,
                "first_name": "Tao",
                "last_name": "Xu",
                "grade_lvl": 11,
                "completed_events": []
            },
            {
                "id": 5,
                "first_name": "Tara",
                "last_name": "Jones",
                "grade_lvl": 12,
                "completed_events": []
            },
            {
                "id": 6,
                "first_name": "Darcy",
                "last_name": "Olsson",
                "grade_lvl": 12,
                "completed_events": []
            },
            {
                "id": 7,
                "first_name": "Issac",
                "last_name": "Henderson",
                "grade_lvl": 9,
                "completed_events": []
            },
            {
                "id": 8,
                "first_name": "Tori",
                "last_name": "Spring",
                "grade_lvl": 9,
                "completed_events": []
            }
        ]).to_string();

        // events data
        let events_json = json!([
            {
                "id": 1,
                "name": "Basketball tournament",
                "points": 10
            },
            {
                "id": 2,
                "name": "Football game",
                "points": 12
            },
            {
                "id": 3,
                "name": "Track and Field Meet",
                "points": 18
            },
            {
                "id": 4,
                "name": "Volleyball game",
                "points": 25
            },
            {
                "id": 5,
                "name": "Swim meet",
                "points": 5
            },
            {
                "id": 6,
                "name": "Science fair",
                "points": 15
            },
            {
                "id": 7,
                "name": "Art show",
                "points": 24
            },
            {
                "id": 8,
                "name": "Debate competition",
                "points": 18
            },
            {
                "id": 9,
                "name": "Talent show",
                "points": 5
            },
            {
                "id": 10,
                "name": "Math Olympiad",
                "points": 4
            }
        ]).to_string();

        // construct data
        self.students = serde_json::from_str(&students_json).unwrap();
        self.events = serde_json::from_str(&events_json).unwrap();
        self.write_json();
    }
}
