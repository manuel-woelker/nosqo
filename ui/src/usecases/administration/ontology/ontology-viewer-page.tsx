import { Badge, SimpleGrid } from "@mantine/core";
import { useEffect } from "react";
import { NosqoEmptyState } from "../../../common/components/nosqo-empty-state";
import { NosqoErrorAlert } from "../../../common/components/nosqo-error-alert";
import { NosqoPanel } from "../../../common/components/nosqo-panel";
import { NosqoTextInput } from "../../../common/components/nosqo-text-input";
import { ApiError, fetchOntologyText } from "../../../infrastructure/api/api-client";
import { parseOntologyNosqo, normalizeOntologyValue } from "./parse-ontology-nosqo";
import type { OntologyEntity } from "./ontology-types";
import { useOntologyViewerStore } from "./use-ontology-viewer-store";

export function OntologyViewerPage() {
  const entities = useOntologyViewerStore((state) => state.entities);
  const errorMessage = useOntologyViewerStore((state) => state.errorMessage);
  const filterText = useOntologyViewerStore((state) => state.filterText);
  const isLoading = useOntologyViewerStore((state) => state.isLoading);
  const reset = useOntologyViewerStore((state) => state.reset);
  const selectedEntityId = useOntologyViewerStore((state) => state.selectedEntityId);
  const selectEntity = useOntologyViewerStore((state) => state.selectEntity);
  const setEntities = useOntologyViewerStore((state) => state.setEntities);
  const setErrorMessage = useOntologyViewerStore((state) => state.setErrorMessage);
  const setFilterText = useOntologyViewerStore((state) => state.setFilterText);
  const setIsLoading = useOntologyViewerStore((state) => state.setIsLoading);

  useEffect(() => {
    async function loadOntology() {
      setIsLoading(true);
      setErrorMessage(null);

      try {
        const ontologyText = await fetchOntologyText();
        setEntities(parseOntologyNosqo(ontologyText));
      } catch (error) {
        if (error instanceof ApiError) {
          setErrorMessage(error.message);
        } else if (error instanceof Error) {
          setErrorMessage(error.message);
        } else {
          setErrorMessage("The ontology viewer failed for an unknown reason.");
        }
        setEntities([]);
      } finally {
        setIsLoading(false);
      }
    }

    void loadOntology();

    return () => {
      reset();
    };
  }, [reset, setEntities, setErrorMessage, setIsLoading]);

  const normalizedFilter = filterText.trim().toLowerCase();
  const filteredEntities = entities.filter((entity) => {
    if (normalizedFilter.length === 0) {
      return true;
    }

    return (
      entity.displayName.toLowerCase().includes(normalizedFilter) ||
      entity.subject.toLowerCase().includes(normalizedFilter)
    );
  });
  const selectedEntity =
    filteredEntities.find((entity) => entity.id === selectedEntityId) ??
    entities.find((entity) => entity.id === selectedEntityId) ??
    filteredEntities[0] ??
    null;

  return (
    <section className="panel-stack">
      <NosqoPanel className="panel-stack">
        <div className="ontology-header">
          <div className="panel-stack">
            <p className="kicker">Administration / Model</p>
            <h1 className="page-title">Ontology</h1>
            <p className="body-copy">
              Browse the ontology as a read-only projection of the server&apos;s ontology
              statements. This viewer is for inspection, not mutation.
            </p>
          </div>
          <Badge color="teal" radius="xl" variant="light">
            Read-only
          </Badge>
        </div>

        <SimpleGrid cols={{ base: 1, sm: 3 }}>
          <OntologyStatCard label="Types" value={countEntitiesByKind(entities, "type")} />
          <OntologyStatCard label="Predicates" value={countEntitiesByKind(entities, "predicate")} />
          <OntologyStatCard label="Entities" value={entities.length} />
        </SimpleGrid>
      </NosqoPanel>

      {errorMessage ? <NosqoErrorAlert message={errorMessage} /> : null}

      {isLoading ? <p className="hint">Loading ontology snapshot...</p> : null}

      {!isLoading && !errorMessage && entities.length === 0 ? (
        <NosqoEmptyState
          body="The server returned no ontology entities. Check whether ontology knowledge is loaded."
          title="No ontology entities"
        />
      ) : null}

      {!isLoading && !errorMessage && entities.length > 0 ? (
        <div className="ontology-layout">
          <NosqoPanel className="panel-stack ontology-sidebar">
            <div className="panel-stack">
              <NosqoTextInput
                label="Filter ontology entities"
                onChange={(event) => setFilterText(event.currentTarget.value)}
                placeholder="Filter by name or id"
                value={filterText}
              />
              <p className="hint">
                {filteredEntities.length} of {entities.length} ontology entities visible
              </p>
            </div>

            {filteredEntities.length === 0 ? (
              <NosqoEmptyState
                body="Try a broader filter to find ontology types or predicates."
                title="No matching ontology entities"
              />
            ) : (
              <div className="ontology-entity-list" role="list" aria-label="Ontology entities">
                {filteredEntities.map((entity) => {
                  const isSelected = selectedEntity?.id === entity.id;

                  return (
                    <button
                      aria-pressed={isSelected}
                      className={
                        isSelected ? "ontology-entity ontology-entity--selected" : "ontology-entity"
                      }
                      key={entity.subject}
                      onClick={() => selectEntity(entity.id)}
                      type="button"
                    >
                      <span className="ontology-entity__kind">{entity.kind}</span>
                      <strong className="ontology-entity__name">{entity.displayName}</strong>
                      <span className="ontology-entity__subject">{entity.subject}</span>
                    </button>
                  );
                })}
              </div>
            )}
          </NosqoPanel>

          <NosqoPanel className="panel-stack">
            {selectedEntity ? (
              <OntologyDetail entity={selectedEntity} />
            ) : (
              <NosqoEmptyState
                body="Select an ontology entity to inspect its relationships and raw nosqo block."
                title="Nothing selected"
              />
            )}
          </NosqoPanel>
        </div>
      ) : null}
    </section>
  );
}

function OntologyDetail({ entity }: { entity: OntologyEntity }) {
  const overviewItems = [
    { label: "Identifier", value: entity.subject },
    { label: "Kind", value: entity.kind },
    { label: "Display name", value: entity.displayName },
  ];

  return (
    <div className="panel-stack">
      <div className="panel-stack">
        <div className="ontology-detail__header">
          <div>
            <p className="kicker">Selected entity</p>
            <h2 className="ontology-detail__title">{entity.displayName}</h2>
          </div>
          <Badge color={entity.kind === "predicate" ? "cyan" : "teal"} radius="xl" variant="light">
            {entity.kind}
          </Badge>
        </div>

        <p className="body-copy">
          {entity.description ?? "No description is available for this ontology entity yet."}
        </p>
      </div>

      <section className="ontology-section">
        <h3>Overview</h3>
        <dl className="ontology-overview-grid">
          {overviewItems.map((item) => (
            <div className="ontology-overview-item" key={item.label}>
              <dt>{item.label}</dt>
              <dd>{item.value}</dd>
            </div>
          ))}
        </dl>
      </section>

      <section className="ontology-section">
        <h3>Relationships</h3>
        <div className="ontology-relationship-grid">
          <OntologyTagList
            emptyLabel="No parent relationships"
            items={entity.parents}
            title="Parents"
          />
          <OntologyTagList
            emptyLabel="No child relationships"
            items={entity.children}
            title="Children"
          />
        </div>
      </section>

      {entity.kind === "type" ? (
        <section className="ontology-section">
          <h3>Properties</h3>
          <OntologyTagList
            emptyLabel="No attribute predicates declared"
            items={entity.attributes}
            title="Allowed attributes"
          />
        </section>
      ) : null}

      {entity.kind === "predicate" ? (
        <section className="ontology-section">
          <h3>Target Types</h3>
          <OntologyTagList
            emptyLabel="No target types declared"
            items={entity.targetTypes}
            title="Allowed target types"
          />
        </section>
      ) : null}

      <section className="ontology-section">
        <h3>Statements</h3>
        <div className="table-shell">
          <table>
            <thead>
              <tr>
                <th scope="col">Predicate</th>
                <th scope="col">Object</th>
              </tr>
            </thead>
            <tbody>
              {entity.statements.map((statement) => (
                <tr key={`${statement.predicate}-${statement.object}`}>
                  <td>{statement.predicate}</td>
                  <td>{normalizeOntologyValue(statement.object)}</td>
                </tr>
              ))}
            </tbody>
          </table>
        </div>
      </section>

      <section className="ontology-section">
        <h3>Raw nosqo block</h3>
        <pre className="code-block">{entity.rawBlock}</pre>
      </section>
    </div>
  );
}

function OntologyTagList({
  emptyLabel,
  items,
  title,
}: {
  emptyLabel: string;
  items: string[];
  title: string;
}) {
  return (
    <div className="panel-stack">
      <p className="hint">{title}</p>
      {items.length === 0 ? (
        <p className="body-copy">{emptyLabel}</p>
      ) : (
        <div className="ontology-tag-list">
          {items.map((item) => (
            <span className="ontology-tag" key={item}>
              {item}
            </span>
          ))}
        </div>
      )}
    </div>
  );
}

function OntologyStatCard({ label, value }: { label: string; value: number }) {
  return (
    <div className="feature-card">
      <p className="kicker">{label}</p>
      <p className="ontology-stat">{value}</p>
    </div>
  );
}

function countEntitiesByKind(entities: OntologyEntity[], kind: OntologyEntity["kind"]): number {
  return entities.filter((entity) => entity.kind === kind).length;
}
