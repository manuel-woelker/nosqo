import { FormEvent, useEffect, useState } from "react";
import { ApiError, fetchStatements } from "../lib/api-client";

interface StatementFiltersForm {
  subject: string;
  predicate: string;
  object: string;
}

const EMPTY_FILTERS: StatementFiltersForm = {
  subject: "",
  predicate: "",
  object: "",
};

export function StatementsPage() {
  const [filters, setFilters] = useState<StatementFiltersForm>(EMPTY_FILTERS);
  const [errorMessage, setErrorMessage] = useState<string | null>(null);
  const [isLoading, setIsLoading] = useState(true);
  const [statementText, setStatementText] = useState("");

  async function loadStatements(nextFilters: StatementFiltersForm) {
    setIsLoading(true);
    setErrorMessage(null);

    try {
      const nextStatementText = await fetchStatements(nextFilters);
      setStatementText(nextStatementText);
    } catch (error) {
      if (error instanceof ApiError) {
        setErrorMessage(error.message);
      } else {
        setErrorMessage("The statement browser request failed for an unknown reason.");
      }
      setStatementText("");
    } finally {
      setIsLoading(false);
    }
  }

  useEffect(() => {
    void loadStatements(EMPTY_FILTERS);
  }, []);

  function handleSubmit(event: FormEvent<HTMLFormElement>) {
    event.preventDefault();
    void loadStatements(filters);
  }

  return (
    <section className="panel stack">
      <div className="stack">
        <p className="kicker">Statements</p>
        <h2>Inspect the raw statement store.</h2>
        <p className="body-copy">
          This is the blunt instrument screen. That is fine. Raw output is useful while the API and
          data model are still settling.
        </p>
      </div>

      <form className="filters-grid" onSubmit={handleSubmit}>
        <label className="field" htmlFor="subject">
          <span>Subject</span>
          <input
            id="subject"
            name="subject"
            onChange={(event) =>
              setFilters((current) => ({
                ...current,
                subject: event.target.value,
              }))
            }
            placeholder="frodo_baggins"
            value={filters.subject}
          />
        </label>

        <label className="field" htmlFor="predicate">
          <span>Predicate</span>
          <input
            id="predicate"
            name="predicate"
            onChange={(event) =>
              setFilters((current) => ({
                ...current,
                predicate: event.target.value,
              }))
            }
            placeholder="isA"
            value={filters.predicate}
          />
        </label>

        <label className="field" htmlFor="object">
          <span>Object</span>
          <input
            id="object"
            name="object"
            onChange={(event) =>
              setFilters((current) => ({
                ...current,
                object: event.target.value,
              }))
            }
            placeholder="#Person"
            value={filters.object}
          />
        </label>

        <div className="toolbar toolbar--filters">
          <button disabled={isLoading} type="submit">
            {isLoading ? "Loading..." : "Load statements"}
          </button>
        </div>
      </form>

      {errorMessage ? (
        <div className="error-banner" role="alert">
          {errorMessage}
        </div>
      ) : null}

      {isLoading ? <p className="hint">Loading statements...</p> : null}

      {!isLoading && !errorMessage && statementText.trim().length === 0 ? (
        <div className="empty-state">
          <h3>No statements matched</h3>
          <p>Try a broader filter or check whether the knowledge base is loaded.</p>
        </div>
      ) : null}

      {!isLoading && statementText.trim().length > 0 ? (
        <pre className="code-block">{statementText}</pre>
      ) : null}
    </section>
  );
}
