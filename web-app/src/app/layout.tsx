import '@/styles/globals.scss';

export const metadata = {
  title: 'Activity Tracker',
}

export default function RootLayout({
  children,
}: {
  children: React.ReactNode
}) {
  return (
    <html lang="en">
      <head></head>
      <body>
        {children}
      </body>
    </html>
  );
}
