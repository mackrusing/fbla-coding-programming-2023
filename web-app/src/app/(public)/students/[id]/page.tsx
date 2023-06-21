import styles from "@/styles/info.module.scss";
import { retrieveStudent, Student, Event } from "@/lib/api";
import { notFound } from 'next/navigation';

export default async function StudentPage({ params }: { params: { id: string } }) {
  const student = await retrieveStudent(Number(params.id));
  if (!student) {
    notFound();
  }

  let events = student.completedEvents.map(event => {
    return (
      <div className={styles.row} key={event.id}>
        <p className={styles.eventName}>{event.name}</p>
        <p className={styles.eventPoints}>{event.points}</p>
      </div>
    );
  });

  if (events.length === 0) {
    events = (
      <div className={styles.noneRow}>
        <p>attend events to earn points</p>
      </div>
    );
  }

  const info = (
    <p>
      grade, id?, rank, total points
    </p>
  );

  return(
    <main className={styles.main}>
      {/* name and rank */}
      <h1 className={styles.name}>
        {student.firstName + " " + student.lastName}&nbsp;
        <span className={styles.rank}>(#{student.rank})</span>
      </h1>

      {/* id, grade, points */}
      <p className={styles.info}>
        <span className={styles.title}>points</span> {student.totalPoints} <span className={styles.separator}>&#183;</span> <span className={styles.title}>id</span> {student.id} <span className={styles.separator}>&#183;</span> <span className={styles.title}>grade</span> {student.gradeLvl}
      </p>

      {/* list of completed events */}
      <div className={styles.completedEvents}>
        <div className={styles.headRow}>
          <p className={styles.eventName}>name</p>
          <p className={styles.eventPoints}>pts</p>
        </div>
        {events}
        <div className={styles.footRow}>
          <p className={styles.eventName}></p>
          <p className={styles.eventPoints}><span className={styles.total}>total: </span>{student.totalPoints}</p>
        </div>
      </div>
    </main>
  );
}
