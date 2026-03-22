export function NosqoErrorAlert({ message }: { message: string }) {
  return (
    <div className="nosqo-error-alert" role="alert">
      {message}
    </div>
  );
}
