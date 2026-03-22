import { ErrorComponentProps, Link } from "@tanstack/react-router";

export function RouteErrorPage({ error }: ErrorComponentProps) {
  return (
    <section className="panel stack">
      <p className="kicker">Route error</p>
      <h2>The UI tripped over itself.</h2>
      <p className="body-copy">{error.message}</p>
      <Link className="button-link" to="/">
        Back to safety
      </Link>
    </section>
  );
}
