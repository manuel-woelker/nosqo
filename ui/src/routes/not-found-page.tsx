import { Link } from "@tanstack/react-router";

export function NotFoundPage() {
  return (
    <section className="panel stack">
      <p className="kicker">404</p>
      <h2>This route wandered off the graph.</h2>
      <p className="body-copy">
        The UI only ships a few routes right now. The rest are still vapor.
      </p>
      <Link className="button-link" to="/">
        Return home
      </Link>
    </section>
  );
}
