import { type FormEvent, useEffect, useState } from "react";
import { NosqoButton } from "../../../common/components/nosqo-button";
import { NosqoEmptyState } from "../../../common/components/nosqo-empty-state";
import { NosqoErrorAlert } from "../../../common/components/nosqo-error-alert";
import { NosqoPanel } from "../../../common/components/nosqo-panel";
import { NosqoSelect } from "../../../common/components/nosqo-select";
import { NosqoTable, type NosqoTableRow } from "../../../common/components/nosqo-table";
import { NosqoTextInput } from "../../../common/components/nosqo-text-input";
import {
  ApiError,
  type EntityDetailResponse,
  type EntitySearchResult,
  fetchEntityDetail,
  fetchEntitySearch,
  fetchOntologyStatementJson,
} from "../../../infrastructure/api/api-client";
import type { OntologyEntity } from "../ontology/ontology-types";
import { transformOntologyStatementJson } from "../ontology/transform-ontology-statement-json";

interface EntityBrowserType {
  attributes: EntityBrowserAttribute[];
  description: string | null;
  displayName: string;
  subject: string;
}

interface EntityBrowserAttribute {
  displayName: string;
  predicateId: string;
}

export function EntityBrowserPage() {
  const [ontologyEntities, setOntologyEntities] = useState<OntologyEntity[]>([]);
  const [ontologyErrorMessage, setOntologyErrorMessage] = useState<string | null>(null);
  const [isLoadingOntology, setIsLoadingOntology] = useState(true);
  const [selectedTypeSubject, setSelectedTypeSubject] = useState("");
  const [filterValues, setFilterValues] = useState<Record<string, string>>({});
  const [searchResults, setSearchResults] = useState<EntitySearchResult[] | null>(null);
  const [searchErrorMessage, setSearchErrorMessage] = useState<string | null>(null);
  const [isSearching, setIsSearching] = useState(false);
  const [selectedEntityId, setSelectedEntityId] = useState<string | null>(null);
  const [entityDetail, setEntityDetail] = useState<EntityDetailResponse | null>(null);
  const [detailErrorMessage, setDetailErrorMessage] = useState<string | null>(null);
  const [isLoadingDetail, setIsLoadingDetail] = useState(false);

  useEffect(() => {
    let isCancelled = false;

    async function loadOntology() {
      setIsLoadingOntology(true);
      setOntologyErrorMessage(null);

      try {
        const ontologyStatementJson = await fetchOntologyStatementJson();
        const entities = transformOntologyStatementJson(ontologyStatementJson);
        const nextAvailableTypes = createEntityBrowserTypes(entities);

        if (!isCancelled) {
          setOntologyEntities(entities);
          setSelectedTypeSubject((current) => {
            if (
              current.length > 0 &&
              nextAvailableTypes.some((entityType) => entityType.subject === current)
            ) {
              return current;
            }

            return nextAvailableTypes[0]?.subject ?? "";
          });
        }
      } catch (error) {
        if (isCancelled) {
          return;
        }

        setOntologyEntities([]);
        setOntologyErrorMessage(
          error instanceof ApiError
            ? error.message
            : error instanceof Error
              ? error.message
              : "The entity browser failed to load the ontology.",
        );
      } finally {
        if (!isCancelled) {
          setIsLoadingOntology(false);
        }
      }
    }

    void loadOntology();

    return () => {
      isCancelled = true;
    };
  }, []);

  const availableTypes = createEntityBrowserTypes(ontologyEntities);
  const selectedType =
    availableTypes.find((entityType) => entityType.subject === selectedTypeSubject) ?? null;

  useEffect(() => {
    if (!selectedEntityId) {
      setEntityDetail(null);
      setDetailErrorMessage(null);
      setIsLoadingDetail(false);
      return;
    }

    const entityId = selectedEntityId;
    let isCancelled = false;

    async function loadEntityDetail() {
      setIsLoadingDetail(true);
      setDetailErrorMessage(null);

      try {
        const detail = await fetchEntityDetail(entityId);

        if (!isCancelled) {
          setEntityDetail(detail);
        }
      } catch (error) {
        if (isCancelled) {
          return;
        }

        setEntityDetail(null);
        setDetailErrorMessage(
          error instanceof ApiError
            ? error.message
            : error instanceof Error
              ? error.message
              : "The entity detail request failed for an unknown reason.",
        );
      } finally {
        if (!isCancelled) {
          setIsLoadingDetail(false);
        }
      }
    }

    void loadEntityDetail();

    return () => {
      isCancelled = true;
    };
  }, [selectedEntityId]);

  async function handleSubmit(event: FormEvent<HTMLFormElement>) {
    event.preventDefault();

    if (!selectedTypeSubject) {
      return;
    }

    setIsSearching(true);
    setSearchErrorMessage(null);
    setEntityDetail(null);
    setDetailErrorMessage(null);

    try {
      const filters = Object.fromEntries(
        Object.entries(filterValues)
          .map(([key, value]) => [key, value.trim()])
          .filter(([, value]) => value.length > 0),
      );
      const response = await fetchEntitySearch({
        filters,
        type: selectedTypeSubject,
      });

      setSearchResults(response.results);
      setSelectedEntityId(response.results[0]?.id ?? null);
    } catch (error) {
      setSearchResults(null);
      setSelectedEntityId(null);
      setSearchErrorMessage(
        error instanceof ApiError
          ? error.message
          : error instanceof Error
            ? error.message
            : "The entity search failed for an unknown reason.",
      );
    } finally {
      setIsSearching(false);
    }
  }

  function handleTypeChange(nextTypeSubject: string) {
    setSelectedTypeSubject(nextTypeSubject);
    setFilterValues({});
    setSearchResults(null);
    setSearchErrorMessage(null);
    setSelectedEntityId(null);
    setEntityDetail(null);
    setDetailErrorMessage(null);
  }

  return (
    <section className="entity-browser-page">
      <div className="admin-page__header">
        <h1 className="admin-page__title">Entity Browser</h1>
      </div>

      <p className="body-copy">
        Search real entities by type and exact attribute values, then inspect the full attribute set
        without dropping into raw NQL.
      </p>

      <div className="entity-browser-layout">
        <NosqoPanel className="panel-stack entity-browser-pane entity-browser-query-pane">
          <div className="panel-stack">
            <p className="kicker">Query</p>
            <p className="body-copy">
              Pick a type, fill any exact-match attributes you care about, and run search.
            </p>
          </div>

          {ontologyErrorMessage ? <NosqoErrorAlert message={ontologyErrorMessage} /> : null}
          {isLoadingOntology ? <p className="hint">Loading ontology metadata...</p> : null}

          {!isLoadingOntology && availableTypes.length === 0 && !ontologyErrorMessage ? (
            <NosqoEmptyState
              body="The ontology does not expose any queryable types yet."
              title="No entity types"
            />
          ) : null}

          {availableTypes.length > 0 ? (
            <form className="panel-stack" onSubmit={handleSubmit}>
              <NosqoSelect
                label="Type"
                name="entity-type"
                onChange={(event) => handleTypeChange(event.currentTarget.value)}
                options={availableTypes.map((entityType) => ({
                  label: entityType.displayName,
                  value: entityType.subject,
                }))}
                value={selectedTypeSubject}
              />

              {selectedType?.description ? (
                <p className="hint entity-browser-type-copy">{selectedType.description}</p>
              ) : null}

              {selectedType && selectedType.attributes.length > 0 ? (
                <div className="panel-stack">
                  {selectedType.attributes.map((attribute) => (
                    <NosqoTextInput
                      key={attribute.predicateId}
                      label={attribute.displayName}
                      name={`filter-${attribute.predicateId}`}
                      onChange={(event) => {
                        const nextValue = event.currentTarget.value;

                        setFilterValues((current) => ({
                          ...current,
                          [attribute.predicateId]: nextValue,
                        }));
                      }}
                      value={filterValues[attribute.predicateId] ?? ""}
                    />
                  ))}
                </div>
              ) : null}

              {selectedType && selectedType.attributes.length === 0 ? (
                <NosqoEmptyState
                  body="This type does not currently expose any ontology attributes for query inputs."
                  title="No queryable attributes"
                />
              ) : null}

              <div className="toolbar">
                <NosqoButton disabled={isSearching || !selectedTypeSubject} type="submit">
                  {isSearching ? "Searching..." : "Search"}
                </NosqoButton>
                <NosqoButton
                  disabled={isSearching}
                  onClick={() => handleTypeChange(selectedTypeSubject)}
                  type="button"
                >
                  Clear
                </NosqoButton>
              </div>
            </form>
          ) : null}
        </NosqoPanel>

        <div className="entity-browser-results-detail">
          <NosqoPanel className="panel-stack entity-browser-pane entity-browser-results-pane">
            <div className="entity-browser-pane__header">
              <div>
                <p className="kicker">Matches</p>
                <h2 className="entity-browser-pane__title">Entity Results</h2>
              </div>
              {searchResults ? (
                <p className="hint">
                  {searchResults.length} {searchResults.length === 1 ? "entity" : "entities"}
                </p>
              ) : null}
            </div>

            {searchErrorMessage ? <NosqoErrorAlert message={searchErrorMessage} /> : null}

            {searchResults === null && !searchErrorMessage ? (
              <NosqoEmptyState
                body="Run a search to populate the result table."
                title="No results yet"
              />
            ) : null}

            {searchResults && searchResults.length === 0 ? (
              <NosqoEmptyState
                body="The entity search completed successfully, but nothing matched those exact values."
                title="No matching entities"
              />
            ) : null}

            {searchResults && searchResults.length > 0 ? (
              <div className="table-shell">
                <table>
                  <caption className="sr-only">Matching entities</caption>
                  <thead>
                    <tr>
                      <th scope="col">Label</th>
                      <th scope="col">Entity Id</th>
                    </tr>
                  </thead>
                  <tbody>
                    {searchResults.map((result) => (
                      <tr
                        className={
                          result.id === selectedEntityId
                            ? "entity-browser-result-row--selected"
                            : undefined
                        }
                        key={result.id}
                      >
                        <td>
                          <button
                            className="entity-browser-result-button"
                            onClick={() => setSelectedEntityId(result.id)}
                            type="button"
                          >
                            {result.label}
                          </button>
                        </td>
                        <td>
                          <code>{result.nosqoId}</code>
                        </td>
                      </tr>
                    ))}
                  </tbody>
                </table>
              </div>
            ) : null}
          </NosqoPanel>

          <NosqoPanel className="panel-stack entity-browser-pane entity-browser-detail-pane">
            <div className="entity-browser-pane__header">
              <div>
                <p className="kicker">Detail</p>
                <h2 className="entity-browser-pane__title">Entity Details</h2>
              </div>
            </div>

            {detailErrorMessage ? <NosqoErrorAlert message={detailErrorMessage} /> : null}
            {isLoadingDetail ? <p className="hint">Loading entity details...</p> : null}

            {!isLoadingDetail && !entityDetail && selectedEntityId === null ? (
              <NosqoEmptyState
                body="Select a matching entity to inspect all of its attributes."
                title="No entity selected"
              />
            ) : null}

            {entityDetail ? (
              <div className="panel-stack">
                <div className="panel-stack entity-browser-detail-summary">
                  <div className="entity-browser-detail-heading">
                    <h3 className="entity-browser-detail-title">{entityDetail.label}</h3>
                    <code>{entityDetail.nosqoId}</code>
                  </div>
                  <p className="hint">
                    Types:{" "}
                    {entityDetail.typeIds.length > 0 ? entityDetail.typeIds.join(", ") : "None"}
                  </p>
                </div>

                <NosqoTable
                  columns={[
                    { id: "attribute", label: "Attribute" },
                    { id: "values", label: "Values" },
                  ]}
                  rows={entityDetail.attributes.map<NosqoTableRow>((attribute) => ({
                    key: attribute.predicateId,
                    cells: [
                      <div className="panel-stack" key={`${attribute.predicateId}-label`}>
                        <strong>{attribute.label}</strong>
                        <code>{attribute.predicateId}</code>
                      </div>,
                      <div
                        className="entity-browser-values"
                        key={`${attribute.predicateId}-values`}
                      >
                        {attribute.values.map((value) => (
                          <code key={`${attribute.predicateId}-${value}`}>{value}</code>
                        ))}
                      </div>,
                    ],
                  }))}
                />
              </div>
            ) : null}
          </NosqoPanel>
        </div>
      </div>
    </section>
  );
}

function createEntityBrowserTypes(ontologyEntities: OntologyEntity[]): EntityBrowserType[] {
  const predicateEntities = new Map(
    ontologyEntities
      .filter((entity) => entity.kind === "predicate")
      .map((entity) => [entity.subject, entity] as const),
  );

  return ontologyEntities
    .filter((entity) => entity.kind === "type" && entity.subject !== "#Type")
    .map((entity) => ({
      subject: entity.subject,
      displayName: entity.displayName,
      description: entity.description,
      attributes: entity.attributes
        .map((predicateId) => {
          const predicate = predicateEntities.get(predicateId);

          return {
            predicateId,
            displayName: predicate?.displayName ?? predicateId.replace(/^~/, ""),
          };
        })
        .sort((left, right) => left.displayName.localeCompare(right.displayName)),
    }))
    .sort((left, right) => left.displayName.localeCompare(right.displayName));
}
