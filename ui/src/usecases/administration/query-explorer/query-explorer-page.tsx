import { type FormEvent, useState } from "react";
import { NosqoButton } from "../../../common/components/nosqo-button";
import { NosqoErrorAlert } from "../../../common/components/nosqo-error-alert";
import { NosqoPanel } from "../../../common/components/nosqo-panel";
import { NosqoTextarea } from "../../../common/components/nosqo-textarea";
import {
  ApiError,
  type QueryResponse,
  fetchNqlQuery,
} from "../../../infrastructure/api/api-client";
import { QueryResultsTable } from "./query-results-table";

const DEFAULT_QUERY = `match
?person ~isA #Person
?person ~name ?name
return
?person ?name
`;

export function QueryExplorerPage() {
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
    <section className="admin-page">
      <div className="admin-page__header">
        <h1 className="admin-page__title">Query Explorer</h1>
      </div>

      <NosqoPanel className="admin-page__panel">
        <div className="admin-page__body">
          <div className="panel-stack">
            <p className="body-copy">
              Run queries against the loaded knowledge base without pretending the graph is a
              spreadsheet. Good query UX beats decorative chrome every time.
            </p>
          </div>

          <form className="panel-stack" onSubmit={handleSubmit}>
            <NosqoTextarea
              label="Query text"
              name="nql-query"
              onChange={(event) => setQueryText(event.currentTarget.value)}
              value={queryText}
            />

            <div className="toolbar">
              <NosqoButton disabled={isLoading} type="submit">
                {isLoading ? "Running query..." : "Run query"}
              </NosqoButton>
              <p className="hint">
                Posts plain text to <code>/api/v1/query</code>.
              </p>
            </div>
          </form>

          {errorMessage ? <NosqoErrorAlert message={errorMessage} /> : null}

          {result ? <QueryResultsTable result={result} /> : null}
        </div>
      </NosqoPanel>
    </section>
  );
}
