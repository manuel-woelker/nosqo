import type { MouseEventHandler, ReactNode } from "react";

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
    <button className="nosqo-button" disabled={disabled} onClick={onClick} type={type}>
      {children}
    </button>
  );
}
