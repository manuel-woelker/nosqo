import type { ChangeEventHandler, InputHTMLAttributes } from "react";

interface NosqoTextInputProps extends Omit<
  InputHTMLAttributes<HTMLInputElement>,
  "children" | "size"
> {
  label: string;
}

export function NosqoTextInput({ id, label, name, onChange, ...props }: NosqoTextInputProps) {
  const inputId = id ?? name ?? label.toLowerCase().replace(/\s+/g, "-");

  return (
    <label className="field" htmlFor={inputId}>
      <span>{label}</span>
      <input
        className="nosqo-input"
        id={inputId}
        name={name}
        onChange={onChange as ChangeEventHandler<HTMLInputElement> | undefined}
        {...props}
      />
    </label>
  );
}
