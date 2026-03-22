import type { MouseEventHandler, ReactNode } from "react";
import { Button } from "@mantine/core";

export function NosqoButton({
  children,
  disabled,
  onClick,
  type,
}: {
  children: ReactNode;
  disabled?: boolean;
  onClick?: MouseEventHandler<HTMLButtonElement>;
  type?: "button" | "reset" | "submit";
}) {
  return (
    <Button disabled={disabled} onClick={onClick} radius="xl" type={type} variant="light">
      {children}
    </Button>
  );
}
