import type { ReactNode } from "react";
import { Paper } from "@mantine/core";

export function NosqoPanel({ children, className }: { children: ReactNode; className?: string }) {
  const nextClassName = className ? `nosqo-panel ${className}` : "nosqo-panel";

  return (
    <Paper className={nextClassName} p="md" radius="lg" withBorder>
      {children}
    </Paper>
  );
}
