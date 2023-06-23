import AddEventForm from "./add-event.tsx";
import AddStudentForm from "./add-student.tsx";
import EventsList from "./events-list.tsx";
import styles from "./page.module.scss";

export default function AdminDashboard() {
  return (
    <main className={styles.main}>
      <div className={styles.formContainer}>
        <h2>Add Student</h2>
        <AddStudentForm />
      </div>

      <div className={styles.formContainer}>
        <h2>Add Event</h2>
        <AddEventForm />
      </div>

      <h2>All Events</h2>
      <EventsList />
    </main>
  );
}
