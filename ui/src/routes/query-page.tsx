import { FormEvent, useState } from "react";
import { QueryResultsTable } from "../components/query-results-table";
import { ApiError, QueryResponse, fetchNqlQuery } from "../lib/api-client";

const DEFAULT_QUERY = `match
?person ~isA #Person
?person ~name ?name
return
?person ?name
`;

export function QueryPage() {
  const [queryText, setQueryText] = useState(DEFAULT_QUERY);
  const [errorMessage, setErrorMessage] = useState<string | null>(null);
  const [isLoading, setIsLoading] = useState(false);
  const [result, setResult] = useState<QueryResponse | null>(null);

  async function handleSubmit(event: FormEvent<HTMLFormElement>) {
    event.preventDefault();
    setIsLoading(true);
    setErrorMessage(null);

    try {
      const nextResult = await fetchNqlQuery(queryText);
      setResult(nextResult);
    } catch (error) {
      if (error instanceof ApiError) {
        setErrorMessage(error.message);
      } else {
        setErrorMessage("The query request failed for an unknown reason.");
      }
      setResult(null);
    } finally {
      setIsLoading(false);
    }
  }

  return (
    <section className="panel stack">
      <div className="stack">
        <p className="kicker">NQL</p>
        <h2>Run a query against the loaded knowledge base.</h2>
        <p className="body-copy">
          Keep this screen brutally simple. Good query UX beats decorative chrome every time.
        </p>
      </div>

      <form className="stack" onSubmit={handleSubmit}>
        <label className="field" htmlFor="nql-query">
          <span>Query text</span>
          <textarea
            id="nql-query"
            name="nql-query"
            onChange={(event) => setQueryText(event.target.value)}
            rows={10}
            value={queryText}
          />
        </label>

        <div className="toolbar">
          <button disabled={isLoading} type="submit">
            {isLoading ? "Running query..." : "Run query"}
          </button>
          <p className="hint">
            Posts plain text to <code>/api/v1/query</code>.
          </p>
        </div>
      </form>

      {errorMessage ? (
        <div className="error-banner" role="alert">
          {errorMessage}
        </div>
      ) : null}

      {result ? <QueryResultsTable result={result} /> : null}
    </section>
  );
}
