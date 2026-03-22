import { Alert } from "@mantine/core";

export function NosqoErrorAlert({ message }: { message: string }) {
  return (
    <Alert className="nosqo-error-alert" color="red" radius="lg" role="alert" variant="light">
      {message}
    </Alert>
  );
}
