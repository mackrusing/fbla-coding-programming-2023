import styles from "./nav.module.scss";
import Link from 'next/link';

export function Nav() {
  return (
    <nav className={styles.nav}>
      <ul>
        <li>
          <Link href="/leaderboard">leaderboard</Link>
        </li>
      </ul>
    </nav>
  );
}

export function AuthNav() {
  return (
    <nav className={styles.nav}>
      <ul>
        <li>
          <Link href="/admin/dashboard">dashboard</Link>
        </li>
      </ul>
    </nav>
  );
}
