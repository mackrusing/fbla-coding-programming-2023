"use client"

// import { retrieveStudent, Student, Event } from "@/lib/api";
import { notFound } from "next/navigation";
import { retrieveServerData, logEvent } from "@/lib/api";
import infoStyles from "@/styles/info.module.scss";
import styles from "./page.module.scss";

// export function LogForm()

export default async function AdminEventPage({ params }: { params: { id: string } }) {
  const data = await retrieveServerData();
  const event = data.events.find(element => element.id === Number(params.id));

  if (!event) {
    notFound();
  }

  async function handleLog(e, id: string) {
    console.log(id);
    const res = await logEvent(params.id, id);
    console.log(res);
    if (res.success) {
      e.target.parentNode.remove();
      

      // data.students.filter(student => student.id !== Number(id));
    }
  }

  const students = data.students.filter(student => {
    for (const completeEvent of student.completedEvents) {
      if (completeEvent.id === Number(params.id)) {
        return false;
      }
    }
    return true;
  })

  const studentElements = students.map(student => {
    return (
      <div key={student.id} className={styles.row}>
        <p className={styles.id}>{student.id}</p>
        <p className={styles.name}>{student.firstName + " " + student.lastName}</p>
        <p className={styles.add} onClick={async (e) => await handleLog(e, student.id)}>+</p>
      </div>
    );
  });

  return (
    <main className={infoStyles.main}>
      {/* name */}
      <h1 className={infoStyles.name}>{event.name}</h1>

      {/* id, points */}
      <p className={infoStyles.info}><span className={infoStyles.title}>id</span> {event.id} <span className={styles.separator}>&#183;</span> <span className={infoStyles.title}>pts</span> {event.points}</p>

      {/* form */}
      <h2 className={styles.subheading}>Log Attendance</h2>
      <div className={styles.list}>
        <div className={styles.headRow}>
          <p className={styles.id}>id</p>
          <p className={styles.name}>name</p>
          <p className={styles.add}></p>
        </div>
        {studentElements}
      </div>
    </main>
  );
}
