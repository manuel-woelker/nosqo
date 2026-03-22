import { Link } from "@tanstack/react-router";
import { routePaths } from "../../infrastructure/routing/route-paths";

export function HomePage() {
  return (
    <section className="nosqo-panel panel-stack">
      <div className="panel-stack">
        <p className="kicker">Administration</p>
        <h1 className="page-title">Start with the real workflows.</h1>
        <p className="body-copy">
          The first durable shell focuses on the two browser jobs that already matter: running NQL
          and inspecting the statement store. More sections can earn their way into the navigation
          later.
        </p>
      </div>

      <div className="feature-grid">
        <article className="feature-card">
          <p className="kicker">Model</p>
          <h2>Ontology</h2>
          <p>
            Browse ontology types and predicates as a read-only projection of the server&apos;s
            ontology statements.
          </p>
          <Link className="feature-link" to={routePaths.ontology}>
            Open Ontology
          </Link>
        </article>

        <article className="feature-card">
          <p className="kicker">NQL</p>
          <h2>Query Explorer</h2>
          <p>
            Submit multiline NQL to <code>/api/v1/query</code> and inspect the row-oriented JSON
            response without losing the shape of the data.
          </p>
          <Link className="feature-link" to={routePaths.queryExplorer}>
            Open Query Explorer
          </Link>
        </article>

        <article className="feature-card">
          <p className="kicker">Store</p>
          <h2>Statement Browser</h2>
          <p>
            Filter by subject, predicate, and object, then inspect the raw statement output while
            the API and ontology are still settling.
          </p>
          <Link className="feature-link" to={routePaths.statementBrowser}>
            Open Statement Browser
          </Link>
        </article>
      </div>
    </section>
  );
}
