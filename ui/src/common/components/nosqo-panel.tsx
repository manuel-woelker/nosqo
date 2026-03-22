import type { ReactNode } from "react";

export function NosqoPanel({ children, className }: { children: ReactNode; className?: string }) {
  const nextClassName = className ? `nosqo-panel ${className}` : "nosqo-panel";

  return <div className={nextClassName}>{children}</div>;
}
