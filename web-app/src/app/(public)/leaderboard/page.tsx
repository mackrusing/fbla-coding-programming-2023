import styles from "./page.module.scss";
import Link from 'next/link';
import { retrieveServerData, ServerData, Student, Event } from "@/lib/api";

export default async function LeaderboardPage() {
  const serverData = await retrieveServerData();

  const students = serverData.students.map(student => {
    const link = "/students/" + student.id;

    return (
      <div className={styles.row}>
        <p className={styles.rank}>{student.rank}</p>
        <p className={styles.name}><Link href={link}>{student.firstName + " " + student.lastName}</Link></p>
        <p className={styles.points}>{student.totalPoints}</p>
      </div>
    );
  });

  return(
    <main className={styles.main}>
      <div className={styles.headRow}>
        <p className={styles.rank}>#</p>
        <p className={styles.name}>name</p>
        <p className={styles.points}>pts</p>
      </div>
      {students}
    </main>
  );
}
