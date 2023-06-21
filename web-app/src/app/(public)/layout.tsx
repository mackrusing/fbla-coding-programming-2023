import { Nav } from "@/components/nav";
import Footer from "@/components/footer";

export const metadata = {
  title: 'Activity Tracker',
}

export default function PublicLayout({
  children,
}: {
  children: React.ReactNode
}) {
  return (
    <>
      <Nav />
      {children}
      <Footer />
    </>
  );
}
