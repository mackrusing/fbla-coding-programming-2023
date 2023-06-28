const API = "https://api.activity-tracker.mackk.dev";

// get user data from api
//  TODO: handle failed fetch reqs, see inline comments
export async function retrieveServerData(): Promise<ServerData> {
  // fetch api
  let studentsRes: ApiRes<ApiStudent[]> = await (await fetch(API + "/students.json", {cache: "no-cache"})).json(); // TODO: handle errors here
  let eventsRes: ApiRes<Event[]> = await (await fetch(API + "/events.json", {cache: "no-cache"})).json(); // TODO: handle errors here

  // pull data
  const events: Event[] = eventsRes.data;
  const apiStudents: ApiStudent[] = studentsRes.data;

  // convert students
  const students: Student[] = [];
  for (const apiStudent of apiStudents) {

    // convert event ids to events
    const completedEvents: Event[] = [];
    let totalPoints = 0;
    for (const eventId of apiStudent.completed_events) {
      const event = events.find(element => element.id === eventId);
      if (!event) { continue }
      totalPoints += event.points;
      completedEvents.push(event);
    }

    students.push({
      id: apiStudent.id,
      firstName: apiStudent.first_name,
      lastName: apiStudent.last_name,
      gradeLvl: apiStudent.grade_lvl,
      completedEvents: completedEvents,
      totalPoints: totalPoints,
      rank: 0,
    });
  }

  // sort students and determine rank
  students.sort((a, b) => { return b.totalPoints - a.totalPoints})
  for (let i = 0; i < students.length; i++) {
    const currentStudent = students[i];
    if (currentStudent) { currentStudent.rank = i + 1 }
  }

  // create and return server data
  const data: ServerData = {
    students: students,
    events: events,
  }
  return data;
}

// get a single user from the server
export async function retrieveStudent(id: number): Promise<Student | undefined> {
  const serverData = await retrieveServerData();
  const student = serverData.students.find(element => element.id === id);
  return student;
}

// get a single event from the server
export async function retrieveEvent(id: number): Promise<Event | undefined> {
  const serverData = await retrieveServerData();
  const event = serverData.events.find(element => element.id === id);
  return event;
}

// add a student
export async function addStudent(firstName: string, lastName: string, gradeLvl: string) {
  firstName = encodeURIComponent(firstName);
  lastName = encodeURIComponent(lastName);
  gradeLvl = encodeURIComponent(gradeLvl);

  const settings = {
    method: "POST",
    // cache: "no-cache",
    headers: {
      Authorization: "Basic YWRtaW46cGFzcw=="
    }
  }

  const url = API + `/students.json?first_name=${firstName}&last_name=${lastName}&grade_lvl=${gradeLvl}`;

  const res = await (await fetch(url, settings)).json();
  return res;
}

// add an event
export async function addEvent(name: string, points: string) {
  name: encodeURIComponent(name);
  points: encodeURIComponent(points);

  const settings = {
    method: "POST",
    // cache: "no-cache",
    headers: {
      Authorization: "Basic YWRtaW46cGFzcw=="
    }
  };

  const url = API + `/events.json?name=${name}&points=${points}`;

  const res = await (await fetch(url, settings)).json();
  return res;
}

// log an event
export async function logEvent(eventId: string, studentId: string) {
  eventId: encodeURIComponent(eventId);
  studentId: encodeURIComponent(studentId);

  const settings = {
    method: "POST",
    // cache: "no-cache",
    headers: {
      Authorization: "Basic YWRtaW46cGFzcw=="
    }
  };

  const url = API + `/students/${studentId}/completed_events.json?event_id=${eventId}`;

  const res = await (await fetch(url, settings)).json();
  return res;
}

// both student and event data
export interface ServerData {
  students: Student[];
  events: Event[];
}

// a response object returned by the api
interface ApiRes<T> {
  success: boolean;
  message: string;
  data: T
}

// student data as represented by the api
interface ApiStudent {
  id: number;
  first_name: string;
  last_name: string;
  grade_lvl: 9 | 10 | 11 | 12;
  completed_events: number[];
}

// student data as represented in the program
export interface Student {
  id: number;
  firstName: string;
  lastName: string;
  gradeLvl: 9 | 10 | 11 | 12;
  completedEvents: Event[];
  totalPoints: number;
  rank: number;
}

// event data as represented in the program & by the api
export interface Event {
  id: number;
  name: string;
  points: number;
}

