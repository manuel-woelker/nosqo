import { Badge } from "@mantine/core";
import { useEffect } from "react";
import { NosqoEmptyState } from "../../../common/components/nosqo-empty-state";
import { NosqoErrorAlert } from "../../../common/components/nosqo-error-alert";
import { NosqoPanel } from "../../../common/components/nosqo-panel";
import { NosqoTextInput } from "../../../common/components/nosqo-text-input";
import { ApiError, fetchOntologyStatementJson } from "../../../infrastructure/api/api-client";
import type { OntologyEntity } from "./ontology-types";
import { transformOntologyStatementJson } from "./transform-ontology-statement-json";
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
        const ontologyStatementJson = await fetchOntologyStatementJson();
        setEntities(transformOntologyStatementJson(ontologyStatementJson));
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
  const entityNameBySubject = new Map(
    entities.map((entity) => [entity.subject, entity.displayName]),
  );

  return (
    <section className="panel-stack ontology-page">
      <div className="ontology-header">
        <div className="ontology-header__copy">
          <h1 className="ontology-header__title">Ontology</h1>
        </div>
        <div className="ontology-header__meta">
          <p className="hint">
            {countEntitiesByKind(entities, "type")} types,{" "}
            {countEntitiesByKind(entities, "predicate")} predicates
          </p>
          <Badge color="teal" radius="xl" size="sm" variant="light">
            Read-only
          </Badge>
        </div>
      </div>

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
          <NosqoPanel className="ontology-pane ontology-sidebar">
            <div className="panel-stack ontology-pane__header">
              <NosqoTextInput
                label="Filter ontology entities"
                onChange={(event) => setFilterText(event.currentTarget.value)}
                placeholder="Filter by name or identifier"
                value={filterText}
              />
              <p className="hint">
                {filteredEntities.length} of {entities.length} ontology entities visible
              </p>
            </div>

            <div className="ontology-pane__body">
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
                          isSelected
                            ? "ontology-entity ontology-entity--selected"
                            : "ontology-entity"
                        }
                        key={entity.subject}
                        onClick={() => selectEntity(entity.id)}
                        type="button"
                      >
                        <div className="ontology-entity__row">
                          <span className="ontology-entity__kind">
                            {formatKindLabel(entity.kind)}
                          </span>
                          <strong className="ontology-entity__name">{entity.displayName}</strong>
                        </div>
                      </button>
                    );
                  })}
                </div>
              )}
            </div>
          </NosqoPanel>

          <NosqoPanel className="ontology-pane ontology-detail-pane">
            <div className="ontology-pane__body">
              {selectedEntity ? (
                <OntologyDetail entity={selectedEntity} entityNameBySubject={entityNameBySubject} />
              ) : (
                <NosqoEmptyState
                  body="Select an ontology entity to inspect its relationships and raw nosqo block."
                  title="Nothing selected"
                />
              )}
            </div>
          </NosqoPanel>
        </div>
      ) : null}
    </section>
  );
}

function OntologyDetail({
  entity,
  entityNameBySubject,
}: {
  entity: OntologyEntity;
  entityNameBySubject: Map<string, string>;
}) {
  return (
    <div className="panel-stack ontology-detail">
      <div className="panel-stack ontology-detail__hero">
        <div className="ontology-detail__header">
          <Badge
            color={entity.kind === "predicate" ? "cyan" : "teal"}
            radius="xl"
            size="sm"
            variant="light"
          >
            {formatKindLabel(entity.kind)}
          </Badge>
          <h2 className="ontology-detail__title">{entity.displayName}</h2>
          <p className="ontology-detail__meta">{entity.subject}</p>
        </div>

        <p className="body-copy">
          {entity.description ?? "No description is available for this ontology entity yet."}
        </p>
      </div>

      <section className="ontology-section">
        <h3>Relationships</h3>
        <div className="ontology-relationship-grid">
          <OntologyTagList
            emptyLabel="No parent relationships"
            items={entity.parents}
            entityNameBySubject={entityNameBySubject}
            title="Parents"
          />
          <OntologyTagList
            emptyLabel="No child relationships"
            items={entity.children}
            entityNameBySubject={entityNameBySubject}
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
            entityNameBySubject={entityNameBySubject}
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
            entityNameBySubject={entityNameBySubject}
            title="Allowed target types"
          />
        </section>
      ) : null}
    </div>
  );
}

function OntologyTagList({
  emptyLabel,
  entityNameBySubject,
  items,
  title,
}: {
  emptyLabel: string;
  entityNameBySubject: Map<string, string>;
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
              {formatOntologyReference(item, entityNameBySubject)}
            </span>
          ))}
        </div>
      )}
    </div>
  );
}

function countEntitiesByKind(entities: OntologyEntity[], kind: OntologyEntity["kind"]): number {
  return entities.filter((entity) => entity.kind === kind).length;
}

function formatKindLabel(kind: OntologyEntity["kind"]): string {
  if (kind === "predicate") {
    return "Predicate";
  }

  return "Type";
}

function formatOntologyReference(value: string, entityNameBySubject: Map<string, string>): string {
  return (
    entityNameBySubject.get(value) ??
    value.replace(/^[#~@]/, "").replace(/([a-z])([A-Z])/g, "$1 $2")
  );
}
