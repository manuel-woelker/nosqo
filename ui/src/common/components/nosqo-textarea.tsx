import type { ChangeEventHandler, TextareaHTMLAttributes } from "react";

interface NosqoTextareaProps extends Omit<TextareaHTMLAttributes<HTMLTextAreaElement>, "children"> {
  label: string;
  minRows?: number;
}

export function NosqoTextarea({
  id,
  label,
  minRows = 6,
  name,
  onChange,
  rows,
  ...props
}: NosqoTextareaProps) {
  const textareaId = id ?? name ?? label.toLowerCase().replace(/\s+/g, "-");

  return (
    <label className="field" htmlFor={textareaId}>
      <span>{label}</span>
      <textarea
        className="nosqo-input nosqo-textarea"
        id={textareaId}
        name={name}
        onChange={onChange as ChangeEventHandler<HTMLTextAreaElement> | undefined}
        rows={rows ?? minRows}
        {...props}
      />
    </label>
  );
}
