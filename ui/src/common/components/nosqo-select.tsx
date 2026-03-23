import type { ChangeEventHandler, SelectHTMLAttributes } from "react";

interface NosqoSelectOption {
  label: string;
  value: string;
}

interface NosqoSelectProps extends Omit<SelectHTMLAttributes<HTMLSelectElement>, "children"> {
  label: string;
  options: NosqoSelectOption[];
}

export function NosqoSelect({ id, label, name, onChange, options, ...props }: NosqoSelectProps) {
  const selectId = id ?? name ?? label.toLowerCase().replace(/\s+/g, "-");

  return (
    <label className="field" htmlFor={selectId}>
      <span>{label}</span>
      <select
        className="nosqo-input"
        id={selectId}
        name={name}
        onChange={onChange as ChangeEventHandler<HTMLSelectElement> | undefined}
        {...props}
      >
        {options.map((option) => (
          <option key={option.value} value={option.value}>
            {option.label}
          </option>
        ))}
      </select>
    </label>
  );
}
