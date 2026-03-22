import { Link } from "@tanstack/react-router";

export function HomePage() {
  return (
    <section className="panel stack">
      <div className="stack">
        <p className="kicker">Bootstrap target</p>
        <h2>Start with the real workflows.</h2>
        <p className="body-copy">
          This first pass focuses on the two browser jobs that matter right now: running NQL and
          inspecting statements. Everything else can earn its complexity later.
        </p>
      </div>

      <div className="feature-grid">
        <article className="feature-card">
          <h3>Query explorer</h3>
          <p>
            Submit multiline NQL to <code>/api/v1/query</code> and render the row-oriented JSON
            response as a table.
          </p>
          <Link className="feature-link" to="/query">
            Open query explorer
          </Link>
        </article>

        <article className="feature-card">
          <h3>Statement browser</h3>
          <p>
            Filter statements by subject, predicate, and object, then render the server&apos;s nosqo
            text without getting clever.
          </p>
          <Link className="feature-link" to="/statements">
            Open statement browser
          </Link>
        </article>
      </div>
    </section>
  );
}
