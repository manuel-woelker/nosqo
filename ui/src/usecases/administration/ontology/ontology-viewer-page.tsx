import { useEffect } from "react";
import { NosqoEmptyState } from "../../../common/components/nosqo-empty-state";
import { NosqoErrorAlert } from "../../../common/components/nosqo-error-alert";
import { NosqoPanel } from "../../../common/components/nosqo-panel";
import { NosqoTable, type NosqoTableRow } from "../../../common/components/nosqo-table";
import { NosqoTextInput } from "../../../common/components/nosqo-text-input";
import type { OntologyEntity } from "./ontology-types";
import { useOntologyViewerStore } from "./use-ontology-viewer-store";

export function OntologyViewerPage() {
  const entities = useOntologyViewerStore((state) => state.entities);
  const errorMessage = useOntologyViewerStore((state) => state.errorMessage);
  const filterText = useOntologyViewerStore((state) => state.filterText);
  const isLoading = useOntologyViewerStore((state) => state.isLoading);
  const loadOntology = useOntologyViewerStore((state) => state.loadOntology);
  const selectedEntityId = useOntologyViewerStore((state) => state.selectedEntityId);
  const selectEntity = useOntologyViewerStore((state) => state.selectEntity);
  const setFilterText = useOntologyViewerStore((state) => state.setFilterText);

  useEffect(() => {
    void loadOntology();
  }, [loadOntology]);

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
  const entityBySubject = new Map(entities.map((entity) => [entity.subject, entity]));

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
          <span className="nosqo-badge">Read-only</span>
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
                <OntologyDetail
                  entity={selectedEntity}
                  entityBySubject={entityBySubject}
                  entityNameBySubject={entityNameBySubject}
                  onSelectEntity={selectEntity}
                />
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
  entityBySubject,
  entityNameBySubject,
  onSelectEntity,
}: {
  entity: OntologyEntity;
  entityBySubject: Map<string, OntologyEntity>;
  entityNameBySubject: Map<string, string>;
  onSelectEntity: (entityId: string) => void;
}) {
  const sourceTypes = findSourceTypesForPredicate(entity, entityBySubject);

  return (
    <div className="panel-stack ontology-detail">
      <div className="panel-stack ontology-detail__hero">
        <div className="ontology-detail__header">
          <span
            className={
              entity.kind === "predicate" ? "nosqo-badge nosqo-badge--predicate" : "nosqo-badge"
            }
          >
            {formatKindLabel(entity.kind)}
          </span>
          <h2 className="ontology-detail__title">{entity.displayName}</h2>
        </div>

        <p className="body-copy">
          {entity.description ?? "No description is available for this ontology entity yet."}
        </p>
      </div>

      {entity.kind === "type" ? (
        <section className="ontology-section">
          <h3>Allowed attributes</h3>
          {entity.attributes.length === 0 ? (
            <p className="body-copy">No attribute predicates declared.</p>
          ) : (
            <NosqoTable
              className="ontology-reference-table"
              columns={[
                { id: "attribute", label: "Attribute" },
                { id: "description", label: "Description" },
                { id: "targetTypes", label: "Target types" },
              ]}
              rows={entity.attributes.map((attributeSubject) => {
                const attribute = entityBySubject.get(attributeSubject);

                return {
                  key: attributeSubject,
                  cells: [
                    renderEntityLink(
                      attributeSubject,
                      entityBySubject,
                      entityNameBySubject,
                      onSelectEntity,
                    ),
                    attribute?.description ?? "No description.",
                    renderEntityLinkList(
                      attribute?.targetTypes ?? [],
                      entityBySubject,
                      entityNameBySubject,
                      onSelectEntity,
                    ),
                  ],
                };
              })}
            />
          )}
        </section>
      ) : null}

      {entity.kind === "predicate" ? (
        <>
          <section className="ontology-section">
            <h3>Source types</h3>
            {sourceTypes.length === 0 ? (
              <p className="body-copy">No source types declared.</p>
            ) : (
              <NosqoTable
                className="ontology-reference-table"
                columns={[
                  { id: "type", label: "Type" },
                  { id: "description", label: "Description" },
                ]}
                rows={sourceTypes.map((sourceType) => createEntityReferenceRow(sourceType))}
              />
            )}
          </section>

          <section className="ontology-section ontology-section--spaced">
            <h3>Target types</h3>
            {entity.targetTypes.length === 0 ? (
              <p className="body-copy">No target types declared.</p>
            ) : (
              <NosqoTable
                className="ontology-reference-table"
                columns={[
                  { id: "type", label: "Type" },
                  { id: "description", label: "Description" },
                ]}
                rows={entity.targetTypes.map((targetTypeSubject) =>
                  createEntityReferenceRow(
                    entityBySubject.get(targetTypeSubject) ?? targetTypeSubject,
                  ),
                )}
              />
            )}
          </section>
        </>
      ) : null}
    </div>
  );

  function createEntityReferenceRow(reference: OntologyEntity | string): NosqoTableRow {
    const subject = typeof reference === "string" ? reference : reference.subject;
    const referencedEntity =
      typeof reference === "string" ? (entityBySubject.get(reference) ?? null) : reference;

    return {
      key: subject,
      cells: [
        renderEntityLink(subject, entityBySubject, entityNameBySubject, onSelectEntity),
        referencedEntity?.description ?? "No description.",
      ],
    };
  }
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

function resolveEntityId(value: string, entityBySubject: Map<string, OntologyEntity>): string {
  return entityBySubject.get(value)?.id ?? value.replace(/^[#~@]/, "");
}

function findSourceTypesForPredicate(
  entity: OntologyEntity,
  entityBySubject: Map<string, OntologyEntity>,
): OntologyEntity[] {
  if (entity.kind !== "predicate") {
    return [];
  }

  return Array.from(entityBySubject.values())
    .filter(
      (candidate) => candidate.kind === "type" && candidate.attributes.includes(entity.subject),
    )
    .sort((left, right) => left.displayName.localeCompare(right.displayName));
}

function renderEntityLink(
  subject: string,
  entityBySubject: Map<string, OntologyEntity>,
  entityNameBySubject: Map<string, string>,
  onSelectEntity: (entityId: string) => void,
) {
  return (
    <button
      className="ontology-inline-link"
      onClick={() => onSelectEntity(resolveEntityId(subject, entityBySubject))}
      type="button"
    >
      {formatOntologyReference(subject, entityNameBySubject)}
    </button>
  );
}

function renderEntityLinkList(
  subjects: string[],
  entityBySubject: Map<string, OntologyEntity>,
  entityNameBySubject: Map<string, string>,
  onSelectEntity: (entityId: string) => void,
) {
  if (subjects.length === 0) {
    return <span className="hint">None</span>;
  }

  return (
    <div className="ontology-inline-links">
      {subjects.map((subject) => (
        <button
          className="ontology-inline-link ontology-inline-link--secondary"
          key={subject}
          onClick={() => onSelectEntity(resolveEntityId(subject, entityBySubject))}
          type="button"
        >
          {formatOntologyReference(subject, entityNameBySubject)}
        </button>
      ))}
    </div>
  );
}
