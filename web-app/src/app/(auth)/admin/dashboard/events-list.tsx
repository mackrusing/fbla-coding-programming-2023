import { retrieveServerData } from "@/lib/api";
import Link from "next/link";
import styles from "./events-list.module.scss";

export default async function EventsList() {
  const data = await retrieveServerData();
  const events = data.events.map(event => {
    const link = "/admin/events/" + event.id;

    return (
      <div key={event.id} className={styles.row}>
        <p className={styles.id}>{event.id}</p>
        <p className={styles.name}><Link href={link}>{event.name}</Link></p>
        <p className={styles.points}>{event.points}</p>
      </div>
    );
  });

  return (
    <div className={styles.list}>
      <div className={styles.headRow}>
        <p className={styles.id}>id</p>
        <p className={styles.name}>name</p>
        <p className={styles.points}>pts</p>
      </div>
      {events}
    </div>
  );
}
