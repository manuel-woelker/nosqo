import { type FormEvent, useEffect, useState } from "react";
import { NosqoButton } from "../../../common/components/nosqo-button";
import { NosqoEmptyState } from "../../../common/components/nosqo-empty-state";
import { NosqoErrorAlert } from "../../../common/components/nosqo-error-alert";
import { NosqoPanel } from "../../../common/components/nosqo-panel";
import { NosqoTextInput } from "../../../common/components/nosqo-text-input";
import { ApiError, fetchStatements } from "../../../infrastructure/api/api-client";

interface StatementFiltersForm {
  object: string;
  predicate: string;
  subject: string;
}

const EMPTY_FILTERS: StatementFiltersForm = {
  subject: "",
  predicate: "",
  object: "",
};

export function StatementBrowserPage() {
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
    <section className="admin-page">
      <div className="admin-page__header">
        <h1 className="admin-page__title">Statement Browser</h1>
      </div>

      <NosqoPanel className="admin-page__panel">
        <div className="admin-page__body">
          <div className="panel-stack">
            <p className="body-copy">
              Inspect the raw statement store without getting clever. This is the blunt instrument
              screen, and that is fine while the ontology and API are still moving.
            </p>
          </div>

          <form className="filters-grid" onSubmit={handleSubmit}>
            <NosqoTextInput
              label="Subject"
              name="subject"
              onChange={(event) =>
                setFilters((current) => ({
                  ...current,
                  subject: event.currentTarget.value,
                }))
              }
              placeholder="frodo_baggins"
              value={filters.subject}
            />

            <NosqoTextInput
              label="Predicate"
              name="predicate"
              onChange={(event) =>
                setFilters((current) => ({
                  ...current,
                  predicate: event.currentTarget.value,
                }))
              }
              placeholder="isA"
              value={filters.predicate}
            />

            <NosqoTextInput
              label="Object"
              name="object"
              onChange={(event) =>
                setFilters((current) => ({
                  ...current,
                  object: event.currentTarget.value,
                }))
              }
              placeholder="#Person"
              value={filters.object}
            />

            <div className="toolbar toolbar--filters">
              <NosqoButton disabled={isLoading} type="submit">
                {isLoading ? "Loading..." : "Load statements"}
              </NosqoButton>
            </div>
          </form>

          {errorMessage ? <NosqoErrorAlert message={errorMessage} /> : null}

          {isLoading ? <p className="hint">Loading statements...</p> : null}

          {!isLoading && !errorMessage && statementText.trim().length === 0 ? (
            <NosqoEmptyState
              body="Try a broader filter or check whether the knowledge base is loaded."
              title="No statements matched"
            />
          ) : null}

          {!isLoading && statementText.trim().length > 0 ? (
            <pre className="code-block admin-page__code-block">{statementText}</pre>
          ) : null}
        </div>
      </NosqoPanel>
    </section>
  );
}
