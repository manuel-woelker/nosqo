import { Textarea, type TextareaProps } from "@mantine/core";

export function NosqoTextarea(props: TextareaProps) {
  return <Textarea autosize minRows={6} radius="lg" {...props} />;
}
