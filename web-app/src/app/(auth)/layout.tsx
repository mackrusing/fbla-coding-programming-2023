import { AuthNav } from "@/components/nav";

export const metadata = {
  title: 'Activity Tracker | Admin',
}

export default function AuthLayout({
  children,
}: {
  children: React.ReactNode
}) {
  return (
    <>
      <AuthNav />
      {children}
    </>
  );
}
