import { retrieveStudent, Student, Event } from "@/lib/api";
import { notFound } from "next/navigation";
import styles from "./page.module.scss";

export default async function AdminStudentPage({ params }: { params: { id: string } }) {
  notFound(); // temp

  const student = await retrieveStudent(Number(params.id));
  if (!student) {
    notFound();
  }

  return (
    <main>
      <h1 className={styles.name}>{student.firstName + " " + student.lastName}</h1>
      <div className={styles.info}>
        <div className={styles.firstName}>
        </div>
      </div>
    </main>
  );
}
